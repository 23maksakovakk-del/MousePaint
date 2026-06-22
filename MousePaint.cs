// MousePaint.cs - Рисовалка с мышкой на C# (WinForms)
using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.Windows.Forms;

public class MousePaint : Form
{
    private Bitmap bitmap;
    private Graphics g;
    private Point prevPoint;
    private Color color = Color.Black;
    private int brushSize = 5;
    private bool isDrawing = false;
    private PictureBox pictureBox;

    public MousePaint()
    {
        Text = "🖌️ MousePaint - C#";
        Size = new Size(900, 700);
        bitmap = new Bitmap(800, 600);
        using (var g = Graphics.FromImage(bitmap))
            g.Clear(Color.White);

        pictureBox = new PictureBox { Dock = DockStyle.Fill, Image = bitmap, BackColor = Color.White };
        pictureBox.MouseDown += (s, e) => {
            prevPoint = e.Location;
            isDrawing = true;
        };
        pictureBox.MouseMove += (s, e) => {
            if (!isDrawing) return;
            Point cur = e.Location;
            using (var g = Graphics.FromImage(bitmap))
            {
                g.SmoothingMode = System.Drawing.Drawing2D.SmoothingMode.AntiAlias;
                using (Pen pen = new Pen(color, brushSize)) { pen.StartCap = pen.EndCap = System.Drawing.Drawing2D.LineCap.Round;
                    g.DrawLine(pen, prevPoint, cur); }
                prevPoint = cur;
                pictureBox.Invalidate();
            }
        };
        pictureBox.MouseUp += (s, e) => isDrawing = false;
        Controls.Add(pictureBox);

        // Toolbar
        ToolStrip toolbar = new ToolStrip();
        ToolStripButton brushBtn = new ToolStripButton("🖌️ Кисть");
        brushBtn.Click += (s, e) => { /* цвет уже выбран */ };
        toolbar.Items.Add(brushBtn);
        ToolStripButton eraserBtn = new ToolStripButton("🧽 Ластик");
        eraserBtn.Click += (s, e) => color = Color.White;
        toolbar.Items.Add(eraserBtn);
        ToolStripButton colorBtn = new ToolStripButton("Цвет");
        colorBtn.Click += (s, e) => {
            ColorDialog cd = new ColorDialog();
            if (cd.ShowDialog() == DialogResult.OK) color = cd.Color;
        };
        toolbar.Items.Add(colorBtn);
        ToolStripLabel sizeLabel = new ToolStripLabel("Толщина:");
        toolbar.Items.Add(sizeLabel);
        NumericUpDown sizeBox = new NumericUpDown { Minimum = 1, Maximum = 20, Value = 5, Width = 40 };
        sizeBox.ValueChanged += (s, e) => brushSize = (int)sizeBox.Value;
        toolbar.Items.Add(new ToolStripControlHost(sizeBox));
        ToolStripButton clearBtn = new ToolStripButton("🗑️ Очистить");
        clearBtn.Click += (s, e) => {
            using (var g = Graphics.FromImage(bitmap))
                g.Clear(Color.White);
            pictureBox.Invalidate();
        };
        toolbar.Items.Add(clearBtn);
        ToolStripButton saveBtn = new ToolStripButton("💾 Сохранить");
        saveBtn.Click += (s, e) => {
            SaveFileDialog sfd = new SaveFileDialog { Filter = "PNG|*.png" };
            if (sfd.ShowDialog() == DialogResult.OK)
                bitmap.Save(sfd.FileName, ImageFormat.Png);
        };
        toolbar.Items.Add(saveBtn);
        Controls.Add(toolbar);
        toolbar.Dock = DockStyle.Top;
    }

    [STAThread]
    static void Main() { Application.EnableVisualStyles(); Application.Run(new MousePaint()); }
}
