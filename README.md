# ship_strength_monitoring_system

Система расчета прочности судна (прибор контроля прочности).

Назначение прибора контроля прочности:
 - задание оператором схемы загрузки судна;
 - построение эпюр внешних и внутренних силовых факторов;
 - расчет прочности судна в текущем состоянии загрузки;


Блок-схема расчета прочности корабля:

```mermaid
graph TD;
Strength --> NormalStress
NormalStress -.-> Strength
NormalStress --> BendingMoment;
BendingMoment -.-> NormalStress;
BendingMoment-->ShearForce;
ShearForce -.->BendingMoment;
ShearForce-->TotalShipLoad;
TotalShipLoad -.-> ShearForce;
TotalShipLoad-->Displacment;
TotalShipLoad-->BuoyantLoad;
BuoyantLoad -.->TotalShipLoad;
Displacment-->Deadweight;
Displacment-->Lightweight;
Displacment -.-> TotalShipLoad;
Lightweight -.->Displacment;
Deadweight -.->Displacment;
```

Вложенность объектов:
``` rust
Strenght {
    NormalStress {
        CrossSections,
        BendingMoment {
            SheareForce {
                TotalShipLoad {
                    Displacement {
                        Deadweight {},
                        Lightweight {
                            Ship{}
                        }
                    },
                    BouyanLoad {
                        Ship {},
                        BonjeanScale {}
                    }
                }
            }
        }
    }
}
```
