/********************************************************************************
** Form generated from reading UI file 'main_window.ui'
**
** Created by: Qt User Interface Compiler version 5.15.3
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_MAIN_WINDOW_H
#define UI_MAIN_WINDOW_H

#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QGridLayout>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QSlider>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_MainWindow
{
public:
    QVBoxLayout *verticalLayout;
    QGridLayout *gridLayout;
    QLabel *label;
    QLineEdit *j1_txt;
    QSlider *j1_slider;
    QGridLayout *gridLayout_3;
    QLabel *label_3;
    QLineEdit *j2_txt;
    QSlider *j2_slider;
    QGridLayout *gridLayout_4;
    QLabel *label_4;
    QLineEdit *j3_txt;
    QSlider *j3_slider;
    QGridLayout *gridLayout_5;
    QLabel *j4_label;
    QLineEdit *j4_txt;
    QSlider *j4_slider;
    QPushButton *random_btn;
    QPushButton *center_btn;

    void setupUi(QWidget *MainWindow)
    {
        if (MainWindow->objectName().isEmpty())
            MainWindow->setObjectName(QString::fromUtf8("MainWindow"));
        MainWindow->resize(402, 355);
        verticalLayout = new QVBoxLayout(MainWindow);
        verticalLayout->setObjectName(QString::fromUtf8("verticalLayout"));
        gridLayout = new QGridLayout();
        gridLayout->setObjectName(QString::fromUtf8("gridLayout"));
        label = new QLabel(MainWindow);
        label->setObjectName(QString::fromUtf8("label"));
        label->setAlignment(Qt::AlignLeading|Qt::AlignLeft|Qt::AlignVCenter);

        gridLayout->addWidget(label, 0, 0, 1, 1);

        j1_txt = new QLineEdit(MainWindow);
        j1_txt->setObjectName(QString::fromUtf8("j1_txt"));
        j1_txt->setAlignment(Qt::AlignCenter);
        j1_txt->setReadOnly(true);

        gridLayout->addWidget(j1_txt, 0, 1, 1, 1);

        j1_slider = new QSlider(MainWindow);
        j1_slider->setObjectName(QString::fromUtf8("j1_slider"));
        j1_slider->setBaseSize(QSize(0, 0));
        QFont font;
        font.setPointSize(14);
        j1_slider->setFont(font);
        j1_slider->setMinimum(0);
        j1_slider->setMaximum(1000);
        j1_slider->setPageStep(1);
        j1_slider->setOrientation(Qt::Horizontal);
        j1_slider->setTickPosition(QSlider::NoTicks);
        j1_slider->setTickInterval(0);

        gridLayout->addWidget(j1_slider, 1, 0, 1, 2);

        gridLayout->setColumnStretch(0, 1);
        gridLayout->setColumnStretch(1, 16);

        verticalLayout->addLayout(gridLayout);

        gridLayout_3 = new QGridLayout();
        gridLayout_3->setObjectName(QString::fromUtf8("gridLayout_3"));
        label_3 = new QLabel(MainWindow);
        label_3->setObjectName(QString::fromUtf8("label_3"));
        label_3->setAlignment(Qt::AlignLeading|Qt::AlignLeft|Qt::AlignVCenter);

        gridLayout_3->addWidget(label_3, 0, 0, 1, 1);

        j2_txt = new QLineEdit(MainWindow);
        j2_txt->setObjectName(QString::fromUtf8("j2_txt"));
        j2_txt->setAlignment(Qt::AlignCenter);
        j2_txt->setReadOnly(true);

        gridLayout_3->addWidget(j2_txt, 0, 1, 1, 1);

        j2_slider = new QSlider(MainWindow);
        j2_slider->setObjectName(QString::fromUtf8("j2_slider"));
        j2_slider->setBaseSize(QSize(0, 0));
        j2_slider->setFont(font);
        j2_slider->setMinimum(0);
        j2_slider->setMaximum(1000);
        j2_slider->setPageStep(1);
        j2_slider->setOrientation(Qt::Horizontal);
        j2_slider->setTickPosition(QSlider::NoTicks);
        j2_slider->setTickInterval(0);

        gridLayout_3->addWidget(j2_slider, 1, 0, 1, 2);

        gridLayout_3->setColumnStretch(0, 1);
        gridLayout_3->setColumnStretch(1, 16);

        verticalLayout->addLayout(gridLayout_3);

        gridLayout_4 = new QGridLayout();
        gridLayout_4->setObjectName(QString::fromUtf8("gridLayout_4"));
        label_4 = new QLabel(MainWindow);
        label_4->setObjectName(QString::fromUtf8("label_4"));
        label_4->setAlignment(Qt::AlignLeading|Qt::AlignLeft|Qt::AlignVCenter);

        gridLayout_4->addWidget(label_4, 0, 0, 1, 1);

        j3_txt = new QLineEdit(MainWindow);
        j3_txt->setObjectName(QString::fromUtf8("j3_txt"));
        j3_txt->setAlignment(Qt::AlignCenter);
        j3_txt->setReadOnly(true);

        gridLayout_4->addWidget(j3_txt, 0, 1, 1, 1);

        j3_slider = new QSlider(MainWindow);
        j3_slider->setObjectName(QString::fromUtf8("j3_slider"));
        j3_slider->setBaseSize(QSize(0, 0));
        j3_slider->setFont(font);
        j3_slider->setMinimum(0);
        j3_slider->setMaximum(1000);
        j3_slider->setPageStep(1);
        j3_slider->setOrientation(Qt::Horizontal);
        j3_slider->setTickPosition(QSlider::NoTicks);
        j3_slider->setTickInterval(0);

        gridLayout_4->addWidget(j3_slider, 1, 0, 1, 2);

        gridLayout_4->setColumnStretch(0, 1);
        gridLayout_4->setColumnStretch(1, 16);

        verticalLayout->addLayout(gridLayout_4);

        gridLayout_5 = new QGridLayout();
        gridLayout_5->setObjectName(QString::fromUtf8("gridLayout_5"));
        j4_label = new QLabel(MainWindow);
        j4_label->setObjectName(QString::fromUtf8("j4_label"));
        j4_label->setAlignment(Qt::AlignLeading|Qt::AlignLeft|Qt::AlignVCenter);

        gridLayout_5->addWidget(j4_label, 0, 0, 1, 1);

        j4_txt = new QLineEdit(MainWindow);
        j4_txt->setObjectName(QString::fromUtf8("j4_txt"));
        j4_txt->setAlignment(Qt::AlignCenter);
        j4_txt->setReadOnly(true);

        gridLayout_5->addWidget(j4_txt, 0, 1, 1, 1);

        j4_slider = new QSlider(MainWindow);
        j4_slider->setObjectName(QString::fromUtf8("j4_slider"));
        j4_slider->setBaseSize(QSize(0, 0));
        j4_slider->setFont(font);
        j4_slider->setMinimum(0);
        j4_slider->setMaximum(1000);
        j4_slider->setPageStep(1);
        j4_slider->setOrientation(Qt::Horizontal);
        j4_slider->setTickPosition(QSlider::NoTicks);
        j4_slider->setTickInterval(0);

        gridLayout_5->addWidget(j4_slider, 1, 0, 1, 2);

        gridLayout_5->setColumnStretch(0, 1);
        gridLayout_5->setColumnStretch(1, 16);

        verticalLayout->addLayout(gridLayout_5);

        random_btn = new QPushButton(MainWindow);
        random_btn->setObjectName(QString::fromUtf8("random_btn"));

        verticalLayout->addWidget(random_btn);

        center_btn = new QPushButton(MainWindow);
        center_btn->setObjectName(QString::fromUtf8("center_btn"));

        verticalLayout->addWidget(center_btn);


        retranslateUi(MainWindow);

        QMetaObject::connectSlotsByName(MainWindow);
    } // setupUi

    void retranslateUi(QWidget *MainWindow)
    {
        MainWindow->setWindowTitle(QCoreApplication::translate("MainWindow", "joint_state_publisher", nullptr));
        label->setText(QCoreApplication::translate("MainWindow", "J1", nullptr));
        label_3->setText(QCoreApplication::translate("MainWindow", "J2", nullptr));
        label_4->setText(QCoreApplication::translate("MainWindow", "J3", nullptr));
        j4_label->setText(QCoreApplication::translate("MainWindow", "J4", nullptr));
        random_btn->setText(QCoreApplication::translate("MainWindow", "Random", nullptr));
        center_btn->setText(QCoreApplication::translate("MainWindow", "Center", nullptr));
    } // retranslateUi

};

namespace Ui {
    class MainWindow: public Ui_MainWindow {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_MAIN_WINDOW_H
