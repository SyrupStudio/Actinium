# include <QApplication>
#include <QMainWindow>
#include <QLabel>
#include <QIcon>
#include <QVBoxLayout>
#include <QWidget>

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    QApplication::setStyle("Fusion");

    app.setApplicationName("Actinium");
    app.setApplicationDisplayName("Actinium");

    QIcon appIcon(":/logo.png");
    if (appIcon.isNull()) {
        appIcon = QIcon(":/logo.png");
    }
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
    label->setAlignment(Qt::AlignCenter);

    QFont font = label->font();
    font.setPointSize(20);
    label->setFont(font);

    layout->addWidget(label);
    window.setCentralWidget(central);

    window.show();
    return app.exec();
}