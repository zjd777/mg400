/****************************************************************************
** Meta object code from reading C++ file 'panel_mg400_controller.hpp'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.3)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../../src/MG400_ROS2/mg400_rviz_plugin/include/mg400_rviz_plugin/panel_mg400_controller.hpp"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'panel_mg400_controller.hpp' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.3. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_mg400_rviz_plugin__Mg400ControllerPanel_t {
    QByteArrayData data[6];
    char stringdata0[104];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_mg400_rviz_plugin__Mg400ControllerPanel_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_mg400_rviz_plugin__Mg400ControllerPanel_t qt_meta_stringdata_mg400_rviz_plugin__Mg400ControllerPanel = {
    {
QT_MOC_LITERAL(0, 0, 39), // "mg400_rviz_plugin::Mg400Contr..."
QT_MOC_LITERAL(1, 40, 4), // "tick"
QT_MOC_LITERAL(2, 45, 0), // ""
QT_MOC_LITERAL(3, 46, 19), // "callbackEnableRobot"
QT_MOC_LITERAL(4, 66, 20), // "callbackDisableRobot"
QT_MOC_LITERAL(5, 87, 16) // "callbackSendMovJ"

    },
    "mg400_rviz_plugin::Mg400ControllerPanel\0"
    "tick\0\0callbackEnableRobot\0"
    "callbackDisableRobot\0callbackSendMovJ"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_mg400_rviz_plugin__Mg400ControllerPanel[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       4,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    0,   34,    2, 0x0a /* Public */,
       3,    0,   35,    2, 0x0a /* Public */,
       4,    0,   36,    2, 0x0a /* Public */,
       5,    0,   37,    2, 0x0a /* Public */,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void mg400_rviz_plugin::Mg400ControllerPanel::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<Mg400ControllerPanel *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->tick(); break;
        case 1: _t->callbackEnableRobot(); break;
        case 2: _t->callbackDisableRobot(); break;
        case 3: _t->callbackSendMovJ(); break;
        default: ;
        }
    }
    (void)_a;
}

QT_INIT_METAOBJECT const QMetaObject mg400_rviz_plugin::Mg400ControllerPanel::staticMetaObject = { {
    QMetaObject::SuperData::link<rviz_common::Panel::staticMetaObject>(),
    qt_meta_stringdata_mg400_rviz_plugin__Mg400ControllerPanel.data,
    qt_meta_data_mg400_rviz_plugin__Mg400ControllerPanel,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *mg400_rviz_plugin::Mg400ControllerPanel::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *mg400_rviz_plugin::Mg400ControllerPanel::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_mg400_rviz_plugin__Mg400ControllerPanel.stringdata0))
        return static_cast<void*>(this);
    return rviz_common::Panel::qt_metacast(_clname);
}

int mg400_rviz_plugin::Mg400ControllerPanel::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = rviz_common::Panel::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 4)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 4;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 4)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 4;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
