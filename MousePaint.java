// MousePaint.java - Рисовалка с мышкой на Java (Swing)
import javax.swing.*;
import java.awt.*;
import java.awt.event.*;
import java.awt.image.BufferedImage;
import java.io.File;
import javax.imageio.ImageIO;

public class MousePaint extends JFrame {
    private BufferedImage image;
    private Graphics2D g2d;
    private int prevX, prevY;
    private Color color = Color.BLACK;
    private int brushSize = 5;
    private boolean isDrawing = false;
    private JPanel canvas;

    public MousePaint() {
        setTitle("🖌️ MousePaint - Java");
        setDefaultCloseOperation(EXIT_ON_CLOSE);
        setSize(900, 700);
        setLocationRelativeTo(null);

        image = new BufferedImage(800, 600, BufferedImage.TYPE_INT_RGB);
        Graphics2D g = image.createGraphics();
        g.setColor(Color.WHITE);
        g.fillRect(0, 0, 800, 600);
        g.dispose();

        canvas = new JPanel() {
            @Override
            protected void paintComponent(Graphics g) {
                super.paintComponent(g);
                g.drawImage(image, 0, 0, null);
            }
        };
        canvas.setPreferredSize(new Dimension(800, 600));
        canvas.setBackground(Color.WHITE);
        canvas.addMouseListener(new MouseAdapter() {
            public void mousePressed(MouseEvent e) {
                prevX = e.getX();
                prevY = e.getY();
                isDrawing = true;
                g2d = image.createGraphics();
            }
            public void mouseReleased(MouseEvent e) {
                isDrawing = false;
                g2d.dispose();
                canvas.repaint();
            }
        });
        canvas.addMouseMotionListener(new MouseMotionAdapter() {
            public void mouseDragged(MouseEvent e) {
                if (!isDrawing) return;
                int x = e.getX(), y = e.getY();
                g2d = image.createGraphics();
                g2d.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON);
                g2d.setColor(color);
                g2d.setStroke(new BasicStroke(brushSize, BasicStroke.CAP_ROUND, BasicStroke.JOIN_ROUND));
                g2d.drawLine(prevX, prevY, x, y);
                prevX = x;
                prevY = y;
                canvas.repaint();
            }
        });

        // Toolbar
        JToolBar toolbar = new JToolBar();
        JButton brushBtn = new JButton("🖌️ Кисть");
        brushBtn.addActionListener(e -> { /* цвет уже выбран */ });
        toolbar.add(brushBtn);
        JButton eraserBtn = new JButton("🧽 Ластик");
        eraserBtn.addActionListener(e -> color = Color.WHITE);
        toolbar.add(eraserBtn);
        JButton colorBtn = new JButton("Цвет");
        colorBtn.addActionListener(e -> {
            Color c = JColorChooser.showDialog(this, "Выберите цвет", color);
            if (c != null) color = c;
        });
        toolbar.add(colorBtn);
        JSlider sizeSlider = new JSlider(1, 20, 5);
        sizeSlider.addChangeListener(e -> brushSize = sizeSlider.getValue());
        toolbar.add(new JLabel("Толщина:"));
        toolbar.add(sizeSlider);
        JButton clearBtn = new JButton("🗑️ Очистить");
        clearBtn.addActionListener(e -> {
            Graphics2D g = image.createGraphics();
            g.setColor(Color.WHITE);
            g.fillRect(0, 0, 800, 600);
            g.dispose();
            canvas.repaint();
        });
        toolbar.add(clearBtn);
        JButton saveBtn = new JButton("💾 Сохранить");
        saveBtn.addActionListener(e -> {
            JFileChooser fc = new JFileChooser();
            if (fc.showSaveDialog(MousePaint.this) == JFileChooser.APPROVE_OPTION) {
                try {
                    ImageIO.write(image, "png", fc.getSelectedFile());
                } catch (Exception ex) { ex.printStackTrace(); }
            }
        });
        toolbar.add(saveBtn);
        add(toolbar, BorderLayout.NORTH);
        add(canvas, BorderLayout.CENTER);
        setVisible(true);
    }

    public static void main(String[] args) {
        SwingUtilities.invokeLater(MousePaint::new);
    }
}
