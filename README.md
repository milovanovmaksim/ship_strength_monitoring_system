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
BendingMoment-.->Strength;
BendingMoment-->ShearForce;
ShearForce -.->BendingMoment;
ShearForce-->TotalShipLoad;
TotalShipLoad -.-> ShearForce;
TotalShipLoad-->BuoyancyLoadIntensity;
BuoyancyLoadIntensity -.->TotalShipLoad;
BuoyancyLoadIntensity-->ShipTrimming;
ShipTrimming-.->BuoyancyLoadIntensity;
ShipTrimming--> LCB;
LCB -.-> ShipTrimming;
LCB--> BonjeanScale;
ShipTrimming--> Displacement;
Displacement -.-> ShipTrimming;
Displacement--> BonjeanScale;
ShipTrimming--> LCG;
LCG -.-> ShipTrimming;
LCG--> DisplacementIntensity
DisplacementIntensity -.-> LCG
DisplacementIntensity-->DeadweightIntensity;
DisplacementIntensity-->LightweightIntensity;
LightweightIntensity -.->DisplacementIntensity;
DeadweightIntensity -.->DisplacementIntensity;
ShipTrimming--> DisplacementTonnage;
DisplacementTonnage -.-> ShipTrimming;
DisplacementTonnage -->Lightweight;
DisplacementTonnage -->Deadweight; 
Lightweight -.-> DisplacementTonnage;
Deadweight -.-> DisplacementTonnage;
BonjeanScale--> Frames;
```
