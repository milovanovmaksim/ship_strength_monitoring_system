///
/// Тип данных элементов теоретического чертежа.
/// Paramenters:
///     LCB: абсцисса центра велечины,
///     WaterlineArea: площадь ватерлинии,
///     LCF: абсцисса центра тяжести ватерлиниии,
///     LongitudinalMetacentricRadius - продольный(большой) метацентрический радиус.
pub(crate) enum HydrostaticTypeData {
    LCB,
    LCF,
    WaterlineArea,
    LongitudinalMetacentricRadius,
}
