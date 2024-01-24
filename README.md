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
TotalShipLoad-->Displacement;
TotalShipLoad-->BuoyantLoad;
BuoyantLoad -.->TotalShipLoad;
Displacement-->Deadweight;
Displacement-->Lightweight;
Displacement -.-> TotalShipLoad;
Lightweight -.->Displacement;
Deadweight -.->Displacement;
```
