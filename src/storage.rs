use crate::models::{
    AttributeCollection, AttributePrimaryIndex, BitFieldCollection, BitFieldPrimaryIndex,
    DirectionCollection, DirectionPrimaryIndex, HolidayCollection, InformationTextCollection,
    InformationTextPrimaryIndex, JourneyPlatformCollection, JourneyPlatformPrimaryIndex,
    PlatformCollection, PlatformPrimaryIndex, StopCollection, StopPrimaryIndex,
    ThroughServiceCollection, TransportCompanyCollection, TransportCompanyPrimaryIndex,
    TransportTypeCollection, TransportTypePrimaryIndex, LineCollection, LinePrimaryIndex,
};

// ------------------------------------------------------------------------------------------------
// --- AttributeData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct AttributeData {
    rows: AttributeCollection,
    primary_index: AttributePrimaryIndex,
}

#[allow(unused)]
impl AttributeData {
    pub fn new(rows: AttributeCollection, primary_index: AttributePrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &AttributeCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &AttributePrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- BitFieldData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct BitFieldData {
    rows: BitFieldCollection,
    primary_index: BitFieldPrimaryIndex,
}

#[allow(unused)]
impl BitFieldData {
    pub fn new(rows: BitFieldCollection, primary_index: BitFieldPrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &BitFieldCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &BitFieldPrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- DirectionData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct DirectionData {
    rows: DirectionCollection,
    primary_index: DirectionPrimaryIndex,
}

#[allow(unused)]
impl DirectionData {
    pub fn new(rows: DirectionCollection, primary_index: DirectionPrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &DirectionCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &DirectionPrimaryIndex {
        &self.primary_index
    }
}
// ------------------------------------------------------------------------------------------------
// --- HolidayData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct HolidayData {
    rows: HolidayCollection,
    // primary_index: HolidayPrimaryIndex,
}

#[allow(unused)]
impl HolidayData {
    pub fn new(rows: HolidayCollection) -> Self {
        Self {
            rows,
            // primary_index,
        }
    }

    pub fn rows(&self) -> &HolidayCollection {
        &self.rows
    }

    // pub fn primary_index(&self) -> &HolidayPrimaryIndex {
    //     &self.primary_index
    // }
}

// ------------------------------------------------------------------------------------------------
// --- InformationTextData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct InformationTextData {
    rows: InformationTextCollection,
    primary_index: InformationTextPrimaryIndex,
}

#[allow(unused)]
impl InformationTextData {
    pub fn new(
        rows: InformationTextCollection,
        primary_index: InformationTextPrimaryIndex,
    ) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &InformationTextCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &InformationTextPrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyPlatformData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct JourneyPlatformData {
    rows: JourneyPlatformCollection,
    primary_index: JourneyPlatformPrimaryIndex,
}

#[allow(unused)]
impl JourneyPlatformData {
    pub fn new(
        rows: JourneyPlatformCollection,
        primary_index: JourneyPlatformPrimaryIndex,
    ) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &JourneyPlatformCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &JourneyPlatformPrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- LineData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct LineData {
    rows: LineCollection,
    primary_index: LinePrimaryIndex,
}

#[allow(unused)]
impl LineData {
    pub fn new(rows: LineCollection, primary_index: LinePrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &LineCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &LinePrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- PlatformData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PlatformData {
    rows: PlatformCollection,
    primary_index: PlatformPrimaryIndex,
}

#[allow(unused)]
impl PlatformData {
    pub fn new(rows: PlatformCollection, primary_index: PlatformPrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &PlatformCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &PlatformPrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- StopData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct StopData {
    rows: StopCollection,
    primary_index: StopPrimaryIndex,
}

#[allow(unused)]
impl StopData {
    pub fn new(rows: StopCollection, primary_index: StopPrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &StopCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &StopPrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- ThroughServiceData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ThroughServiceData {
    rows: ThroughServiceCollection,
    // primary_index: ThroughServicePrimaryIndex,
}

#[allow(unused)]
impl ThroughServiceData {
    pub fn new(rows: ThroughServiceCollection) -> Self {
        Self {
            rows,
            // primary_index,
        }
    }

    pub fn rows(&self) -> &ThroughServiceCollection {
        &self.rows
    }

    // pub fn primary_index(&self) -> &ThroughServicePrimaryIndex {
    //     &self.primary_index
    // }
}

// ------------------------------------------------------------------------------------------------
// --- TransportCompanyData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TransportCompanyData {
    rows: TransportCompanyCollection,
    primary_index: TransportCompanyPrimaryIndex,
}

#[allow(unused)]
impl TransportCompanyData {
    pub fn new(
        rows: TransportCompanyCollection,
        primary_index: TransportCompanyPrimaryIndex,
    ) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &TransportCompanyCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &TransportCompanyPrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- TransportTypeData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TransportTypeData {
    rows: TransportTypeCollection,
    primary_index: TransportTypePrimaryIndex,
}

#[allow(unused)]
impl TransportTypeData {
    pub fn new(rows: TransportTypeCollection, primary_index: TransportTypePrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &TransportTypeCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &TransportTypePrimaryIndex {
        &self.primary_index
    }
}
