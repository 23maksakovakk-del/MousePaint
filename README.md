MousePaint — Рисовалка с мышкой на 7 языках
MousePaint — коллекция из семи независимых реализаций простейшего графического редактора с управлением мышью. Каждая версия работает на своём языке программирования и предоставляет интуитивно понятный интерфейс для рисования, выбора цвета и сохранения рисунка.

✨ Общие возможности
🖱️ Рисование мышью (свободная линия)

🎨 Выбор цвета через палитру или ввод RGB/HEX

📏 Настройка толщины линии

🧽 Ластик (стирание до белого фона)

🗑️ Очистка холста

💾 Сохранение рисунка (в PNG на диск или скачивание в вебе)

🎨 Простые фигуры (опционально): прямоугольник, эллипс

↩️ Отмена/Повтор (в большинстве реализаций)

🖥️ Интерфейсы:

Десктопные GUI: Python (Tkinter), Java (Swing), C# (WinForms)

Веб-приложения: JavaScript (чистый HTML+Canvas), Go, Rust, PHP (сервер + клиент)

📋 Сравнение реализаций
Язык	Интерфейс	Undo/Redo	Ластик	Сохранение	Фигуры
Python	Tkinter GUI	✅	✅	PNG	✅
JavaScript	Веб (Canvas)	✅	✅	скачать PNG	✅
Go	Веб (сервер)	✅ (клиент)	✅	скачать + сервер	✅
Rust	Веб (сервер)	✅ (клиент)	✅	скачать + сервер	✅
Java	Swing GUI	❌	✅	PNG	✅
C#	WinForms GUI	❌	✅	PNG	✅
PHP	Веб (сервер)	✅ (клиент)	✅	скачать + сервер	✅
🚀 Быстрый старт
Python
bash
# Tkinter встроен
python mouse_paint.py
JavaScript (браузер)
Откройте mouse_paint.html в браузере.

Go
bash
go run mouse_paint.go
# Откройте http://localhost:8080
Rust
bash
cargo run
# Откройте http://localhost:8000
Java
bash
javac MousePaint.java && java MousePaint
C#
bash
csc /reference:System.Windows.Forms.dll /reference:System.Drawing.dll MousePaint.cs
MousePaint.exe
PHP
bash
php -S localhost:8000
# Откройте http://localhost:8000/mouse_paint.php
