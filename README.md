# ship_strength_monitoring_system

Cистема контроля прочности корабля (прибор контроля прочности).

Назначение прибора контроля прочности:
 - задание оператором схемы загрузки судна;
 - построение эпюр внешних и внутренних силовых факторов;
 - расчет прочности судна в текущем состоянии загрузки;


Блок-схема расчета прочности корабля:

```mermaid
graph TD;
Strength--> BendingMoment;
BendingMoment -.-> NormalStress;
BendingMoment-->ShearForce;
ShearForce -.->BendingMoment;
ShearForce-->TotalShipLoad;
TotalShipLoad -.-> ShearForce;
TotalShipLoad-->Displacement;
TotalShipLoad-->BuoyantLoad;
BuoyantLoad -.->TotalShipLoad;
Displacement-->Deadweight;
Displacement-->Lightweight;
Displacement -.-> TotalShipLoad;
Lightweight -.->Displacement;
Deadweight -.->Displacement;
```
