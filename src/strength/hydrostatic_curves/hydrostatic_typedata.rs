///
/// Тип данных элементов теоретического чертежа.
/// Paramenters:
///     LCB: абсцисса центра велечины,
///     WaterlineArea: площадь ватерлинии,
///     LCF: абсцисса центра тяжести ватерлиниии,
///     LMR - продольный(большой) метацентрический радиус.
#[derive(Debug)]
pub enum HydrostaticTypeData {
    LCB,
    LCF,
    WaterlineArea,
    LMR,
}
