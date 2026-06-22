// mouse_paint.rs - Рисовалка с мышкой на Rust (веб-сервер)
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use base64::{engine::general_purpose, Engine};
use std::fs;

async fn index() -> HttpResponse {
    let html = r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8"><title>🖌️ MousePaint - Rust</title>
<style>body{font-family:sans-serif;background:#2c3e50;display:flex;justify-content:center;align-items:center;min-height:100vh;margin:0;}
.container{background:#ecf0f1;padding:20px;border-radius:16px;box-shadow:0 10px 30px rgba(0,0,0,0.3);}
.toolbar{display:flex;flex-wrap:wrap;gap:6px;margin-bottom:12px;align-items:center;}
.toolbar button{padding:6px 12px;border:none;border-radius:6px;background:#3498db;color:white;cursor:pointer;}
.toolbar button:hover{background:#2980b9;}
.toolbar input[type="color"]{width:40px;height:40px;border:none;cursor:pointer;}
.toolbar input[type="range"]{width:100px;}
canvas{border:2px solid #bdc3c7;border-radius:8px;cursor:crosshair;background:white;}
.status{margin-top:10px;display:flex;justify-content:space-between;font-size:14px;color:#2c3e50;}
</style></head>
<body>
<div class="container">
<h2>🖌️ MousePaint · Rust</h2>
<div class="toolbar">
<button onclick="setTool('brush')">🖌️ Кисть</button>
<button onclick="setTool('eraser')">🧽 Ластик</button>
<input type="color" id="colorPicker" value="#000000">
<label>Толщина: <input type="range" id="sizeSlider" min="1" max="20" value="5"></label>
<button onclick="undo()">↩️ Отменить</button>
<button onclick="redo()">↪️ Повторить</button>
<button onclick="clearCanvas()">🗑️ Очистить</button>
<button onclick="saveImage()">💾 Сохранить</button>
</div>
<canvas id="canvas" width="800" height="600"></canvas>
<div class="status"><span>Инструмент: <span id="toolDisplay">Кисть</span></span><span>Размер: <span id="sizeDisplay">5</span></span></div>
</div>
<script>
const canvas=document.getElementById('canvas');
const ctx=canvas.getContext('2d');
const colorPicker=document.getElementById('colorPicker');
const sizeSlider=document.getElementById('sizeSlider');
let isDrawing=false,lastX=0,lastY=0,tool='brush',color='#000000',size=5;
let undoStack=[],redoStack=[],MAX_UNDO=20;

function init(){ ctx.fillStyle='#ffffff'; ctx.fillRect(0,0,canvas.width,canvas.height); pushUndo(); updateDisplay(); }
function pushUndo(){ undoStack.push(canvas.toDataURL()); if(undoStack.length>MAX_UNDO) undoStack.shift(); redoStack=[]; }
function restoreFromDataURL(data){ let img=new Image(); img.onload=()=>{ ctx.clearRect(0,0,canvas.width,canvas.height); ctx.drawImage(img,0,0); }; img.src=data; }
function undo(){ if(undoStack.length<2)return; redoStack.push(undoStack.pop()); restoreFromDataURL(undoStack[undoStack.length-1]); }
function redo(){ if(redoStack.length===0)return; let data=redoStack.pop(); undoStack.push(data); restoreFromDataURL(data); }
function setTool(t){ tool=t; document.getElementById('toolDisplay').textContent=t==='brush'?'Кисть':'Ластик'; }
function clearCanvas(){ ctx.fillStyle='#ffffff'; ctx.fillRect(0,0,canvas.width,canvas.height); pushUndo(); }
function saveImage(){ let link=document.createElement('a'); link.download='drawing.png'; link.href=canvas.toDataURL('image/png'); link.click();
    let dataURL=canvas.toDataURL('image/png');
    fetch('/save', {method:'POST', body:'image='+encodeURIComponent(dataURL)});
}
function updateDisplay(){ document.getElementById('sizeDisplay').textContent=size; }
function startDrawing(e){ isDrawing=true; let rect=canvas.getBoundingClientRect(); lastX=e.clientX-rect.left; lastY=e.clientY-rect.top; pushUndo(); }
function draw(e){ if(!isDrawing)return; let rect=canvas.getBoundingClientRect(); let x=e.clientX-rect.left; let y=e.clientY-rect.top;
ctx.beginPath(); ctx.moveTo(lastX,lastY); ctx.lineTo(x,y); ctx.strokeStyle=tool==='eraser'?'#ffffff':color; ctx.lineWidth=size; ctx.lineCap='round'; ctx.stroke(); lastX=x; lastY=y; }
function stopDrawing(e){ isDrawing=false; pushUndo(); }
canvas.addEventListener('mousedown',startDrawing);
canvas.addEventListener('mousemove',draw);
canvas.addEventListener('mouseup',stopDrawing);
canvas.addEventListener('mouseleave',stopDrawing);
colorPicker.oninput=(e)=>{ color=e.target.value; };
sizeSlider.oninput=(e)=>{ size=parseInt(e.target.value); updateDisplay(); };
init();
</script>
</body></html>"#;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn save(web::Form(form): web::Form<serde_json::Value>) -> HttpResponse {
    if let Some(image_data) = form.get("image").and_then(|v| v.as_str()) {
        if let Some(encoded) = image_data.strip_prefix("data:image/png;base64,") {
            if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
                if let Ok(()) = fs::write("drawing.png", decoded) {
                    return HttpResponse::Ok().body("ok");
                }
            }
        }
    }
    HttpResponse::BadRequest().body("error")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/save", web::post().to(save))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
