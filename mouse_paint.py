#!/usr/bin/env python3
# mouse_paint.py - Рисовалка с мышкой на Python (Tkinter)
"""
Поддерживает: рисование кистью, выбор цвета, толщину, ластик, очистку, сохранение PNG.
"""
import tkinter as tk
from tkinter import colorchooser, filedialog, ttk
from PIL import Image, ImageDraw, ImageTk
import io

class MousePaint:
    def __init__(self, root):
        self.root = root
        self.root.title("🖌️ MousePaint - Python")
        self.root.geometry("900x700")
        self.root.resizable(True, True)

        self.color = "#000000"
        self.bg_color = "#ffffff"
        self.size = 5
        self.tool = "brush"  # brush, eraser
        self.last_x = None
        self.last_y = None
        self.undo_stack = []
        self.redo_stack = []
        self.max_undo = 20
        self.drawing = False

        # Холст
        self.canvas = tk.Canvas(self.root, bg=self.bg_color, cursor="cross")
        self.canvas.pack(fill=tk.BOTH, expand=True)
        self.canvas.bind("<Button-1>", self.start_draw)
        self.canvas.bind("<B1-Motion>", self.paint)
        self.canvas.bind("<ButtonRelease-1>", self.stop_draw)

        # Изображение для сохранения
        self.image = Image.new("RGB", (800, 600), self.bg_color)
        self.draw = ImageDraw.Draw(self.image)

        self.create_toolbar()
        self.bind_keys()

    def create_toolbar(self):
        toolbar = tk.Frame(self.root, bg="#f0f0f0", relief=tk.RAISED, bd=2)
        toolbar.pack(side=tk.TOP, fill=tk.X)

        tk.Button(toolbar, text="🖌️ Кисть", command=lambda: self.set_tool("brush")).pack(side=tk.LEFT, padx=2)
        tk.Button(toolbar, text="🧽 Ластик", command=lambda: self.set_tool("eraser")).pack(side=tk.LEFT, padx=2)

        tk.Label(toolbar, text="Цвет:").pack(side=tk.LEFT, padx=(10,0))
        self.color_btn = tk.Button(toolbar, bg=self.color, width=2, command=self.choose_color)
        self.color_btn.pack(side=tk.LEFT, padx=2)

        tk.Label(toolbar, text="Толщина:").pack(side=tk.LEFT, padx=(10,0))
        self.size_var = tk.IntVar(value=5)
        tk.Scale(toolbar, from_=1, to=20, orient=tk.HORIZONTAL, variable=self.size_var,
                 length=100, showvalue=0).pack(side=tk.LEFT)

        tk.Button(toolbar, text="↩️ Отменить", command=self.undo).pack(side=tk.LEFT, padx=5)
        tk.Button(toolbar, text="↪️ Повторить", command=self.redo).pack(side=tk.LEFT, padx=5)
        tk.Button(toolbar, text="🗑️ Очистить", command=self.clear).pack(side=tk.LEFT, padx=5)
        tk.Button(toolbar, text="💾 Сохранить", command=self.save).pack(side=tk.LEFT, padx=5)

    def set_tool(self, tool):
        self.tool = tool

    def choose_color(self):
        color = colorchooser.askcolor(color=self.color)[1]
        if color:
            self.color = color
            self.color_btn.config(bg=color)

    def start_draw(self, event):
        self.last_x = event.x
        self.last_y = event.y
        self.drawing = True
        self.push_undo()

    def paint(self, event):
        if not self.drawing:
            return
        x, y = event.x, event.y
        color = self.bg_color if self.tool == "eraser" else self.color
        size = self.size_var.get()
        self.canvas.create_line(self.last_x, self.last_y, x, y,
                                width=size, fill=color,
                                capstyle=tk.ROUND, smooth=True)
        self.draw.line([self.last_x, self.last_y, x, y], fill=color, width=size)
        self.last_x, self.last_y = x, y

    def stop_draw(self, event):
        self.drawing = False
        self.push_undo()

    def push_undo(self):
        # Сохраняем состояние в память
        with io.BytesIO() as output:
            self.image.save(output, format="PNG")
            data = output.getvalue()
        self.undo_stack.append(data)
        if len(self.undo_stack) > self.max_undo:
            self.undo_stack.pop(0)
        self.redo_stack.clear()

    def undo(self):
        if len(self.undo_stack) < 2:
            return
        self.redo_stack.append(self.undo_stack.pop())
        self.restore_from_data(self.undo_stack[-1])
        self.update_canvas()

    def redo(self):
        if not self.redo_stack:
            return
        data = self.redo_stack.pop()
        self.undo_stack.append(data)
        self.restore_from_data(data)
        self.update_canvas()

    def restore_from_data(self, data):
        img = Image.open(io.BytesIO(data))
        self.image = img
        self.draw = ImageDraw.Draw(self.image)

    def update_canvas(self):
        self.canvas.delete("all")
        # Просто загружаем изображение на холст
        img = self.image.resize((self.canvas.winfo_width(), self.canvas.winfo_height()), Image.Resampling.LANCZOS)
        self.tk_img = ImageTk.PhotoImage(img)
        self.canvas.create_image(0, 0, anchor=tk.NW, image=self.tk_img)

    def clear(self):
        self.canvas.delete("all")
        self.image = Image.new("RGB", (800, 600), self.bg_color)
        self.draw = ImageDraw.Draw(self.image)
        self.push_undo()

    def save(self):
        filepath = filedialog.asksaveasfilename(defaultextension=".png",
                                                filetypes=[("PNG", "*.png"), ("All", "*.*")])
        if filepath:
            self.image.save(filepath, "PNG")

    def bind_keys(self):
        self.root.bind("<Control-z>", lambda e: self.undo())
        self.root.bind("<Control-y>", lambda e: self.redo())
        self.root.bind("<Control-s>", lambda e: self.save())

if __name__ == "__main__":
    root = tk.Tk()
    app = MousePaint(root)
    root.mainloop()
