// Copyright (C) 2026 Syrup Studios
// SPDX-License-Identifier: BSD-3-Clause

#include <QApplication>
#include <QFont>
#include <QIcon>
#include <QLabel>
#include <QMainWindow>
#include <QVBoxLayout>
#include <QWidget>
#include <QMenuBar>
#include <iostream>

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    QApplication::setStyle("Fusion");
    std::cout << "Style set" << std::endl;

    app.setApplicationName("Actinium");
    app.setApplicationDisplayName("Actinium");

    QIcon appIcon("assets/logo.png");
    if (!appIcon.isNull()) {
        app.setWindowIcon(appIcon);
    }

    QMainWindow window;
    window.setWindowTitle("Actinium");
    window.resize(400, 320);
    if (!appIcon.isNull()) {
        window.setWindowIcon(appIcon);
    }

    auto *central = new QWidget(&window);
    auto *layout = new QVBoxLayout(central);
    auto *label = new QLabel("Hello World!");
    std::cout << "Actinium is running." << std::endl;
    label->setAlignment(Qt::AlignCenter);

    QFont font = label->font();
    font.setPointSize(20);
    label->setFont(font);

    layout->addWidget(label);
    window.setCentralWidget(central);

    window.show();
    return app.exec();
}
