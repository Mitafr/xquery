use sea_orm::{entity::prelude::*, DbErr, EntityTrait, RelationTrait, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "sdn")]
pub struct Model {
    #[sea_orm(unique)]
    pub fixed_ref: i32,
    #[sea_orm(primary_key, auto_increment)]
    pub record_id: i32,
    #[sea_orm(unique)]
    pub identity: i32,
    pub partysubtypeid: i32,
    #[sea_orm(column_type = "Text")]
    pub sdn_type: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub gender: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub title: Option<String>,
    pub additional_sanctions_information: Option<i32>,
    pub secondary_sanctions_risks: Option<i32>,
    pub organization_established_date: Option<Date>,
    pub organization_type: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub locode: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub micex_code: Option<String>,
    pub duns_number: Option<i32>,
    pub registration_country: Option<i32>,
    pub prohibited_transactions: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub vessel_call_sign: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub other_vessel_call_sign: Option<String>,
    pub vessel_type: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub vessel_flag: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub vessel_owner: Option<String>,
    pub vessel_tonnage: Option<i32>,
    pub vessel_gross_registered_tonnage: Option<i32>,
    pub other_vessel_type: Option<i32>,
    pub cmic_effective_date: Option<Date>,
    pub cmic_sales_date: Option<Date>,
    pub cmic_listing_date: Option<Date>,
    pub ifca_determination: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_bch: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_bsv: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_btg: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_dash: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_etc: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_eth: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_ltc: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_usdt: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_xbt: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_xmr: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_xrp: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_xvh: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub dca_zec: Option<String>,
    pub sanction_date: Option<Date>,
    pub sanction_status: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub construction_number: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub manufacturer_serial_number: Option<String>,
    pub manufacture_date: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub transpondeur_code: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub previous_tail_number: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub tail_number: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub model: Option<String>,
    pub peesa_information: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub comment: Option<String>,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")", nullable)]
    pub topmaj: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub updated_by: Option<String>,
    pub last_update: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::AdditionalSanctionsInformation",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference7,
    #[sea_orm(
        belongs_to = "super::ref_type::Entity",
        from = "Column::Partysubtypeid",
        to = "super::ref_type::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefType,
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::SecondarySanctionsRisks",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference8,
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::OrganizationType",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference6,
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::ProhibitedTransactions",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference5,
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::VesselType",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference4,
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::PeesaInformation",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference3,
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::OtherVesselType",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference2,
    #[sea_orm(
        belongs_to = "super::ref_country::Entity",
        from = "Column::RegistrationCountry",
        to = "super::ref_country::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefCountry,
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::IfcaDetermination",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference1,
    #[sea_orm(has_many = "super::address_sdn::Entity")]
    AddressSdn,
    #[sea_orm(has_many = "super::aircraft_operator_sdn::Entity")]
    AircraftOperatorSdn,
    #[sea_orm(has_many = "super::name_sdn::Entity")]
    NameSdn,
    #[sea_orm(has_many = "super::bic_sdn::Entity")]
    BicSdn,
    #[sea_orm(has_many = "super::bik_sdn::Entity")]
    BikSdn,
    #[sea_orm(has_many = "super::caatsa235_sdn::Entity")]
    Caatsa235Sdn,
    #[sea_orm(has_many = "super::citizen_sdn::Entity")]
    CitizenSdn,
    #[sea_orm(has_many = "super::ddc_alias_sdn::Entity")]
    DdcAliasSdn,
    #[sea_orm(has_many = "super::dob_identity::Entity")]
    DobIdentity,
    #[sea_orm(has_many = "super::document_identity::Entity")]
    DocumentIdentity,
    #[sea_orm(has_many = "super::email_sdn::Entity")]
    EmailSdn,
    #[sea_orm(has_many = "super::eo13662dd_sdn::Entity")]
    Eo13662ddSdn,
    #[sea_orm(has_many = "super::eo13846inf_sdn::Entity")]
    Eo13846infSdn,
    #[sea_orm(has_many = "super::eo14024dd_sdn::Entity")]
    Eo14024ddSdn,
    #[sea_orm(has_many = "super::equity_ticker_sdn::Entity")]
    EquityTickerSdn,
    #[sea_orm(has_many = "super::former_vessel_flag_sdn::Entity")]
    FormerVesselFlagSdn,
    #[sea_orm(has_many = "super::isin_sdn::Entity")]
    IsinSdn,
    #[sea_orm(has_many = "super::issuer_name_sdn::Entity")]
    IssuerNameSdn,
    #[sea_orm(has_many = "super::nationality_identity::Entity")]
    NationalityIdentity,
    #[sea_orm(has_many = "super::nationality_registration_sdn::Entity")]
    NationalityRegistrationSdn,
    #[sea_orm(has_many = "super::other_vessel_flag_sdn::Entity")]
    OtherVesselFlagSdn,
    #[sea_orm(has_many = "super::phone_number_sdn::Entity")]
    PhoneNumberSdn,
    #[sea_orm(has_many = "super::pob_identity::Entity")]
    PobIdentity,
    #[sea_orm(has_many = "super::relation::Entity")]
    Relation,
    #[sea_orm(has_many = "super::relation_sdn::Entity")]
    RelationSdn,
    #[sea_orm(has_many = "super::sdn_program::Entity")]
    ProgramSdn,
    #[sea_orm(has_many = "super::target_sdn::Entity")]
    TargetSdn,
    #[sea_orm(has_many = "super::website_identity::Entity")]
    WebsiteIdentity,
}

impl Related<super::ref_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefType.def()
    }
}

impl Related<super::ref_country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefCountry.def()
    }
}

impl Related<super::address_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AddressSdn.def()
    }
}

impl Related<super::aircraft_operator_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AircraftOperatorSdn.def()
    }
}

impl Related<super::name_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NameSdn.def()
    }
}

impl Related<super::bic_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BicSdn.def()
    }
}

impl Related<super::bik_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BikSdn.def()
    }
}

impl Related<super::caatsa235_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Caatsa235Sdn.def()
    }
}

impl Related<super::citizen_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CitizenSdn.def()
    }
}

impl Related<super::ddc_alias_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DdcAliasSdn.def()
    }
}

impl Related<super::dob_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DobIdentity.def()
    }
}

impl Related<super::document_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DocumentIdentity.def()
    }
}

impl Related<super::email_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailSdn.def()
    }
}

impl Related<super::eo13662dd_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Eo13662ddSdn.def()
    }
}

impl Related<super::eo13846inf_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Eo13846infSdn.def()
    }
}

impl Related<super::eo14024dd_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Eo14024ddSdn.def()
    }
}

impl Related<super::equity_ticker_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EquityTickerSdn.def()
    }
}

impl Related<super::former_vessel_flag_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormerVesselFlagSdn.def()
    }
}

impl Related<super::isin_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IsinSdn.def()
    }
}

impl Related<super::issuer_name_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IssuerNameSdn.def()
    }
}

impl Related<super::nationality_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NationalityIdentity.def()
    }
}

impl Related<super::nationality_registration_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NationalityRegistrationSdn.def()
    }
}

impl Related<super::other_vessel_flag_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OtherVesselFlagSdn.def()
    }
}

impl Related<super::phone_number_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhoneNumberSdn.def()
    }
}

impl Related<super::pob_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PobIdentity.def()
    }
}

impl Related<super::relation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Relation.def()
    }
}

impl Related<super::relation_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelationSdn.def()
    }
}

impl Related<super::sdn_program::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProgramSdn.def()
    }
}

impl Related<super::target_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TargetSdn.def()
    }
}

impl Related<super::website_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebsiteIdentity.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_by = Set(Some("BATCH".to_owned()));
        Ok(self)
    }
}
