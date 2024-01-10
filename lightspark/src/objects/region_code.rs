
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// The alpha-2 representation of a country, as defined by the ISO 3166-1 standard.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RegionCode {
    /// The code representing the country of Afghanistan.

    #[serde(rename="AF")]
    Af,
    /// The code representing the country of Åland Islands.

    #[serde(rename="AX")]
    Ax,
    /// The code representing the country of Albania.

    #[serde(rename="AL")]
    Al,
    /// The code representing the country of Algeria.

    #[serde(rename="DZ")]
    Dz,
    /// The code representing the country of American Samoa.

    #[serde(rename="AS")]
    As,
    /// The code representing the country of Andorra.

    #[serde(rename="AD")]
    Ad,
    /// The code representing the country of Angola.

    #[serde(rename="AO")]
    Ao,
    /// The code representing the country of Anguilla.

    #[serde(rename="AI")]
    Ai,
    /// The code representing the country of Antarctica.

    #[serde(rename="AQ")]
    Aq,
    /// The code representing the country of Antigua and Barbuda.

    #[serde(rename="AG")]
    Ag,
    /// The code representing the country of Argentina.

    #[serde(rename="AR")]
    Ar,
    /// The code representing the country of Armenia.

    #[serde(rename="AM")]
    Am,
    /// The code representing the country of Aruba.

    #[serde(rename="AW")]
    Aw,
    /// The code representing the country of Australia.

    #[serde(rename="AU")]
    Au,
    /// The code representing the country of Austria.

    #[serde(rename="AT")]
    At,
    /// The code representing the country of Azerbaijan.

    #[serde(rename="AZ")]
    Az,
    /// The code representing the country of Bahamas.

    #[serde(rename="BS")]
    Bs,
    /// The code representing the country of Bahrain.

    #[serde(rename="BH")]
    Bh,
    /// The code representing the country of Bangladesh.

    #[serde(rename="BD")]
    Bd,
    /// The code representing the country of Barbados.

    #[serde(rename="BB")]
    Bb,
    /// The code representing the country of Belarus.

    #[serde(rename="BY")]
    By,
    /// The code representing the country of Belgium.

    #[serde(rename="BE")]
    Be,
    /// The code representing the country of Belize.

    #[serde(rename="BZ")]
    Bz,
    /// The code representing the country of Benin.

    #[serde(rename="BJ")]
    Bj,
    /// The code representing the country of Bermuda.

    #[serde(rename="BM")]
    Bm,
    /// The code representing the country of Bhutan.

    #[serde(rename="BT")]
    Bt,
    /// The code representing the country of The Plurinational State of Bolivia.

    #[serde(rename="BO")]
    Bo,
    /// The code representing the country of Bonaire, Sint Eustatius, and Saba.

    #[serde(rename="BQ")]
    Bq,
    /// The code representing the country of Bosnia and Herzegovina.

    #[serde(rename="BA")]
    Ba,
    /// The code representing the country of Botswana.

    #[serde(rename="BW")]
    Bw,
    /// The code representing the country of Bouvet Island.

    #[serde(rename="BV")]
    Bv,
    /// The code representing the country of Brazil.

    #[serde(rename="BR")]
    Br,
    /// The code representing the country of British Indian Ocean Territory.

    #[serde(rename="IO")]
    Io,
    /// The code representing the country of Brunei Darussalam.

    #[serde(rename="BN")]
    Bn,
    /// The code representing the country of Bulgaria.

    #[serde(rename="BG")]
    Bg,
    /// The code representing the country of Burkina Faso.

    #[serde(rename="BF")]
    Bf,
    /// The code representing the country of Burundi.

    #[serde(rename="BI")]
    Bi,
    /// The code representing the country of Cambodia.

    #[serde(rename="KH")]
    Kh,
    /// The code representing the country of Cameroon.

    #[serde(rename="CM")]
    Cm,
    /// The code representing the country of Canada.

    #[serde(rename="CA")]
    Ca,
    /// The code representing the country of Cape Verde.

    #[serde(rename="CV")]
    Cv,
    /// The code representing the country of Cayman Islands.

    #[serde(rename="KY")]
    Ky,
    /// The code representing the country of Central African Republic.

    #[serde(rename="CF")]
    Cf,
    /// The code representing the country of Chad.

    #[serde(rename="TD")]
    Td,
    /// The code representing the country of Chile.

    #[serde(rename="CL")]
    Cl,
    /// The code representing the country of China.

    #[serde(rename="CN")]
    Cn,
    /// The code representing the country of Christmas Island.

    #[serde(rename="CX")]
    Cx,
    /// The code representing the country of Cocos (Keeling) Islands.

    #[serde(rename="CC")]
    Cc,
    /// The code representing the country of Colombia.

    #[serde(rename="CO")]
    Co,
    /// The code representing the country of Comoros.

    #[serde(rename="KM")]
    Km,
    /// The code representing the country of Congo.

    #[serde(rename="CG")]
    Cg,
    /// The code representing the country of The Democratic Republic of the Congo.

    #[serde(rename="CD")]
    Cd,
    /// The code representing the country of Cook Islands.

    #[serde(rename="CK")]
    Ck,
    /// The code representing the country of Costa Rica.

    #[serde(rename="CR")]
    Cr,
    /// The code representing the country of Côte d'Ivoire.

    #[serde(rename="CI")]
    Ci,
    /// The code representing the country of Croatia.

    #[serde(rename="HR")]
    Hr,
    /// The code representing the country of Cuba.

    #[serde(rename="CU")]
    Cu,
    /// The code representing the country of Curaçao.

    #[serde(rename="CW")]
    Cw,
    /// The code representing the country of Cyprus.

    #[serde(rename="CY")]
    Cy,
    /// The code representing the country of Czech Republic.

    #[serde(rename="CZ")]
    Cz,
    /// The code representing the country of Denmark.

    #[serde(rename="DK")]
    Dk,
    /// The code representing the country of Djibouti.

    #[serde(rename="DJ")]
    Dj,
    /// The code representing the country of Dominica.

    #[serde(rename="DM")]
    Dm,
    /// The code representing the country of Dominican Republic.

    #[serde(rename="DO")]
    Do,
    /// The code representing the country of Ecuador.

    #[serde(rename="EC")]
    Ec,
    /// The code representing the country of Egypt.

    #[serde(rename="EG")]
    Eg,
    /// The code representing the country of El Salvador.

    #[serde(rename="SV")]
    Sv,
    /// The code representing the country of Equatorial Guinea.

    #[serde(rename="GQ")]
    Gq,
    /// The code representing the country of Eritrea.

    #[serde(rename="ER")]
    Er,
    /// The code representing the country of Estonia.

    #[serde(rename="EE")]
    Ee,
    /// The code representing the country of Ethiopia.

    #[serde(rename="ET")]
    Et,
    /// The code representing the country of Falkland Islands (Malvinas).

    #[serde(rename="FK")]
    Fk,
    /// The code representing the country of Faroe Islands.

    #[serde(rename="FO")]
    Fo,
    /// The code representing the country of Fiji.

    #[serde(rename="FJ")]
    Fj,
    /// The code representing the country of Finland.

    #[serde(rename="FI")]
    Fi,
    /// The code representing the country of France.

    #[serde(rename="FR")]
    Fr,
    /// The code representing the country of French Guiana.

    #[serde(rename="GF")]
    Gf,
    /// The code representing the country of French Polynesia.

    #[serde(rename="PF")]
    Pf,
    /// The code representing the country of French Southern Territories.

    #[serde(rename="TF")]
    Tf,
    /// The code representing the country of Gabon.

    #[serde(rename="GA")]
    Ga,
    /// The code representing the country of Gambia.

    #[serde(rename="GM")]
    Gm,
    /// The code representing the country of Georgia.

    #[serde(rename="GE")]
    Ge,
    /// The code representing the country of Germany.

    #[serde(rename="DE")]
    De,
    /// The code representing the country of Ghana.

    #[serde(rename="GH")]
    Gh,
    /// The code representing the country of Gibraltar.

    #[serde(rename="GI")]
    Gi,
    /// The code representing the country of Greece.

    #[serde(rename="GR")]
    Gr,
    /// The code representing the country of Greenland.

    #[serde(rename="GL")]
    Gl,
    /// The code representing the country of Grenada.

    #[serde(rename="GD")]
    Gd,
    /// The code representing the country of Guadeloupe.

    #[serde(rename="GP")]
    Gp,
    /// The code representing the country of Guam.

    #[serde(rename="GU")]
    Gu,
    /// The code representing the country of Guatemala.

    #[serde(rename="GT")]
    Gt,
    /// The code representing the country of Guernsey.

    #[serde(rename="GG")]
    Gg,
    /// The code representing the country of Guinea.

    #[serde(rename="GN")]
    Gn,
    /// The code representing the country of Guinea-Bissau.

    #[serde(rename="GW")]
    Gw,
    /// The code representing the country of Guyana.

    #[serde(rename="GY")]
    Gy,
    /// The code representing the country of Haiti.

    #[serde(rename="HT")]
    Ht,
    /// The code representing the country of Heard Island and McDonald Islands.

    #[serde(rename="HM")]
    Hm,
    /// The code representing the country of Holy See (Vatican City State).

    #[serde(rename="VA")]
    Va,
    /// The code representing the country of Honduras.

    #[serde(rename="HN")]
    Hn,
    /// The code representing the country of Hong Kong.

    #[serde(rename="HK")]
    Hk,
    /// The code representing the country of Hungary.

    #[serde(rename="HU")]
    Hu,
    /// The code representing the country of Iceland.

    #[serde(rename="IS")]
    Is,
    /// The code representing the country of India.

    #[serde(rename="IN")]
    In,
    /// The code representing the country of Indonesia.

    #[serde(rename="ID")]
    Id,
    /// The code representing the country of Islamic Republic of Iran.

    #[serde(rename="IR")]
    Ir,
    /// The code representing the country of Iraq.

    #[serde(rename="IQ")]
    Iq,
    /// The code representing the country of Ireland.

    #[serde(rename="IE")]
    Ie,
    /// The code representing the country of Isle of Man.

    #[serde(rename="IM")]
    Im,
    /// The code representing the country of Israel.

    #[serde(rename="IL")]
    Il,
    /// The code representing the country of Italy.

    #[serde(rename="IT")]
    It,
    /// The code representing the country of Jamaica.

    #[serde(rename="JM")]
    Jm,
    /// The code representing the country of Japan.

    #[serde(rename="JP")]
    Jp,
    /// The code representing the country of Jersey.

    #[serde(rename="JE")]
    Je,
    /// The code representing the country of Jordan.

    #[serde(rename="JO")]
    Jo,
    /// The code representing the country of Kazakhstan.

    #[serde(rename="KZ")]
    Kz,
    /// The code representing the country of Kenya.

    #[serde(rename="KE")]
    Ke,
    /// The code representing the country of Kiribati.

    #[serde(rename="KI")]
    Ki,
    /// The code representing the country of Democratic People's Republic ofKorea.

    #[serde(rename="KP")]
    Kp,
    /// The code representing the country of Republic of Korea.

    #[serde(rename="KR")]
    Kr,
    /// The code representing the country of Kuwait.

    #[serde(rename="KW")]
    Kw,
    /// The code representing the country of Kyrgyzstan.

    #[serde(rename="KG")]
    Kg,
    /// The code representing the country of Lao People's Democratic Republic.

    #[serde(rename="LA")]
    La,
    /// The code representing the country of Latvia.

    #[serde(rename="LV")]
    Lv,
    /// The code representing the country of Lebanon.

    #[serde(rename="LB")]
    Lb,
    /// The code representing the country of Lesotho.

    #[serde(rename="LS")]
    Ls,
    /// The code representing the country of Liberia.

    #[serde(rename="LR")]
    Lr,
    /// The code representing the country of Libya.

    #[serde(rename="LY")]
    Ly,
    /// The code representing the country of Liechtenstein.

    #[serde(rename="LI")]
    Li,
    /// The code representing the country of Lithuania.

    #[serde(rename="LT")]
    Lt,
    /// The code representing the country of Luxembourg.

    #[serde(rename="LU")]
    Lu,
    /// The code representing the country of Macao.

    #[serde(rename="MO")]
    Mo,
    /// The code representing the country of The Former Yugoslav Republic of Macedonia.

    #[serde(rename="MK")]
    Mk,
    /// The code representing the country of Madagascar.

    #[serde(rename="MG")]
    Mg,
    /// The code representing the country of Malawi.

    #[serde(rename="MW")]
    Mw,
    /// The code representing the country of Malaysia.

    #[serde(rename="MY")]
    My,
    /// The code representing the country of Maldives.

    #[serde(rename="MV")]
    Mv,
    /// The code representing the country of Mali.

    #[serde(rename="ML")]
    Ml,
    /// The code representing the country of Malta.

    #[serde(rename="MT")]
    Mt,
    /// The code representing the country of Marshall Islands.

    #[serde(rename="MH")]
    Mh,
    /// The code representing the country of Martinique.

    #[serde(rename="MQ")]
    Mq,
    /// The code representing the country of Mauritania.

    #[serde(rename="MR")]
    Mr,
    /// The code representing the country of Mauritius.

    #[serde(rename="MU")]
    Mu,
    /// The code representing the country of Mayotte.

    #[serde(rename="YT")]
    Yt,
    /// The code representing the country of Mexico.

    #[serde(rename="MX")]
    Mx,
    /// The code representing the country of Federated States ofMicronesia.

    #[serde(rename="FM")]
    Fm,
    /// The code representing the country of Republic of Moldova.

    #[serde(rename="MD")]
    Md,
    /// The code representing the country of Monaco.

    #[serde(rename="MC")]
    Mc,
    /// The code representing the country of Mongolia.

    #[serde(rename="MN")]
    Mn,
    /// The code representing the country of Montenegro.

    #[serde(rename="ME")]
    Me,
    /// The code representing the country of Montserrat.

    #[serde(rename="MS")]
    Ms,
    /// The code representing the country of Morocco.

    #[serde(rename="MA")]
    Ma,
    /// The code representing the country of Mozambique.

    #[serde(rename="MZ")]
    Mz,
    /// The code representing the country of Myanmar.

    #[serde(rename="MM")]
    Mm,
    /// The code representing the country of Namibia.

    #[serde(rename="NA")]
    Na,
    /// The code representing the country of Nauru.

    #[serde(rename="NR")]
    Nr,
    /// The code representing the country of Nepal.

    #[serde(rename="NP")]
    Np,
    /// The code representing the country of Netherlands.

    #[serde(rename="NL")]
    Nl,
    /// The code representing the country of New Caledonia.

    #[serde(rename="NC")]
    Nc,
    /// The code representing the country of New Zealand.

    #[serde(rename="NZ")]
    Nz,
    /// The code representing the country of Nicaragua.

    #[serde(rename="NI")]
    Ni,
    /// The code representing the country of Niger.

    #[serde(rename="NE")]
    Ne,
    /// The code representing the country of Nigeria.

    #[serde(rename="NG")]
    Ng,
    /// The code representing the country of Niue.

    #[serde(rename="NU")]
    Nu,
    /// The code representing the country of Norfolk Island.

    #[serde(rename="NF")]
    Nf,
    /// The code representing the country of Northern Mariana Islands.

    #[serde(rename="MP")]
    Mp,
    /// The code representing the country of Norway.

    #[serde(rename="NO")]
    No,
    /// The code representing the country of Oman.

    #[serde(rename="OM")]
    Om,
    /// The code representing the country of Pakistan.

    #[serde(rename="PK")]
    Pk,
    /// The code representing the country of Palau.

    #[serde(rename="PW")]
    Pw,
    /// The code representing the country of State of Palestine.

    #[serde(rename="PS")]
    Ps,
    /// The code representing the country of Panama.

    #[serde(rename="PA")]
    Pa,
    /// The code representing the country of Papua New Guinea.

    #[serde(rename="PG")]
    Pg,
    /// The code representing the country of Paraguay.

    #[serde(rename="PY")]
    Py,
    /// The code representing the country of Peru.

    #[serde(rename="PE")]
    Pe,
    /// The code representing the country of Philippines.

    #[serde(rename="PH")]
    Ph,
    /// The code representing the country of Pitcairn.

    #[serde(rename="PN")]
    Pn,
    /// The code representing the country of Poland.

    #[serde(rename="PL")]
    Pl,
    /// The code representing the country of Portugal.

    #[serde(rename="PT")]
    Pt,
    /// The code representing the country of Puerto Rico.

    #[serde(rename="PR")]
    Pr,
    /// The code representing the country of Qatar.

    #[serde(rename="QA")]
    Qa,
    /// The code representing the country of Réunion.

    #[serde(rename="RE")]
    Re,
    /// The code representing the country of Romania.

    #[serde(rename="RO")]
    Ro,
    /// The code representing the country of Russian Federation.

    #[serde(rename="RU")]
    Ru,
    /// The code representing the country of Rwanda.

    #[serde(rename="RW")]
    Rw,
    /// The code representing the country of Saint Barthélemy.

    #[serde(rename="BL")]
    Bl,
    /// The code representing the country of Saint Helena  Ascension and Tristan da Cunha.

    #[serde(rename="SH")]
    Sh,
    /// The code representing the country of Saint Kitts and Nevis.

    #[serde(rename="KN")]
    Kn,
    /// The code representing the country of Saint Lucia.

    #[serde(rename="LC")]
    Lc,
    /// The code representing the country of Saint Martin (French part).

    #[serde(rename="MF")]
    Mf,
    /// The code representing the country of Saint Pierre and Miquelon.

    #[serde(rename="PM")]
    Pm,
    /// The code representing the country of Saint Vincent and the Grenadines.

    #[serde(rename="VC")]
    Vc,
    /// The code representing the country of Samoa.

    #[serde(rename="WS")]
    Ws,
    /// The code representing the country of San Marino.

    #[serde(rename="SM")]
    Sm,
    /// The code representing the country of Sao Tome and Principe.

    #[serde(rename="ST")]
    St,
    /// The code representing the country of Saudi Arabia.

    #[serde(rename="SA")]
    Sa,
    /// The code representing the country of Senegal.

    #[serde(rename="SN")]
    Sn,
    /// The code representing the country of Serbia.

    #[serde(rename="RS")]
    Rs,
    /// The code representing the country of Seychelles.

    #[serde(rename="SC")]
    Sc,
    /// The code representing the country of Sierra Leone.

    #[serde(rename="SL")]
    Sl,
    /// The code representing the country of Singapore.

    #[serde(rename="SG")]
    Sg,
    /// The code representing the country of Sint Maarten (Dutch part).

    #[serde(rename="SX")]
    Sx,
    /// The code representing the country of Slovakia.

    #[serde(rename="SK")]
    Sk,
    /// The code representing the country of Slovenia.

    #[serde(rename="SI")]
    Si,
    /// The code representing the country of Solomon Islands.

    #[serde(rename="SB")]
    Sb,
    /// The code representing the country of Somalia.

    #[serde(rename="SO")]
    So,
    /// The code representing the country of South Africa.

    #[serde(rename="ZA")]
    Za,
    /// The code representing the country of South Georgia and the South Sandwich Islands.

    #[serde(rename="GS")]
    Gs,
    /// The code representing the country of South Sudan.

    #[serde(rename="SS")]
    Ss,
    /// The code representing the country of Spain.

    #[serde(rename="ES")]
    Es,
    /// The code representing the country of Sri Lanka.

    #[serde(rename="LK")]
    Lk,
    /// The code representing the country of Sudan.

    #[serde(rename="SD")]
    Sd,
    /// The code representing the country of Suriname.

    #[serde(rename="SR")]
    Sr,
    /// The code representing the country of Svalbard and Jan Mayen.

    #[serde(rename="SJ")]
    Sj,
    /// The code representing the country of Swaziland.

    #[serde(rename="SZ")]
    Sz,
    /// The code representing the country of Sweden.

    #[serde(rename="SE")]
    Se,
    /// The code representing the country of Switzerland.

    #[serde(rename="CH")]
    Ch,
    /// The code representing the country of Syrian Arab Republic.

    #[serde(rename="SY")]
    Sy,
    /// The code representing the country of Taiwan, Province of China.

    #[serde(rename="TW")]
    Tw,
    /// The code representing the country of Tajikistan.

    #[serde(rename="TJ")]
    Tj,
    /// The code representing the country of United Republic of Tanzania.

    #[serde(rename="TZ")]
    Tz,
    /// The code representing the country of Thailand.

    #[serde(rename="TH")]
    Th,
    /// The code representing the country of Timor-Leste.

    #[serde(rename="TL")]
    Tl,
    /// The code representing the country of Togo.

    #[serde(rename="TG")]
    Tg,
    /// The code representing the country of Tokelau.

    #[serde(rename="TK")]
    Tk,
    /// The code representing the country of Tonga.

    #[serde(rename="TO")]
    To,
    /// The code representing the country of Trinidad and Tobago.

    #[serde(rename="TT")]
    Tt,
    /// The code representing the country of Tunisia.

    #[serde(rename="TN")]
    Tn,
    /// The code representing the country of Turkey.

    #[serde(rename="TR")]
    Tr,
    /// The code representing the country of Turkmenistan.

    #[serde(rename="TM")]
    Tm,
    /// The code representing the country of Turks and Caicos Islands.

    #[serde(rename="TC")]
    Tc,
    /// The code representing the country of Tuvalu.

    #[serde(rename="TV")]
    Tv,
    /// The code representing the country of Uganda.

    #[serde(rename="UG")]
    Ug,
    /// The code representing the country of Ukraine.

    #[serde(rename="UA")]
    Ua,
    /// The code representing the country of United Arab Emirates.

    #[serde(rename="AE")]
    Ae,
    /// The code representing the country of United Kingdom.

    #[serde(rename="GB")]
    Gb,
    /// The code representing the country of United States.

    #[serde(rename="US")]
    Us,
    /// The code representing the country of United States Minor Outlying Islands.

    #[serde(rename="UM")]
    Um,
    /// The code representing the country of Uruguay.

    #[serde(rename="UY")]
    Uy,
    /// The code representing the country of Uzbekistan.

    #[serde(rename="UZ")]
    Uz,
    /// The code representing the country of Vanuatu.

    #[serde(rename="VU")]
    Vu,
    /// The code representing the country of Bolivarian Republic of Venezuela.

    #[serde(rename="VE")]
    Ve,
    /// The code representing the country of Viet Nam.

    #[serde(rename="VN")]
    Vn,
    /// The code representing the country of British Virgin Islands.

    #[serde(rename="VG")]
    Vg,
    /// The code representing the country of U.S. Virgin Islands.

    #[serde(rename="VI")]
    Vi,
    /// The code representing the country of Wallis and Futuna.

    #[serde(rename="WF")]
    Wf,
    /// The code representing the country of Western Sahara.

    #[serde(rename="EH")]
    Eh,
    /// The code representing the country of Yemen.

    #[serde(rename="YE")]
    Ye,
    /// The code representing the country of Zambia.

    #[serde(rename="ZM")]
    Zm,
    /// The code representing the country of Zimbabwe.

    #[serde(rename="ZW")]
    Zw,

}

impl Into<Value> for RegionCode {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for RegionCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Af => write!(f, "AF"),
            Self::Ax => write!(f, "AX"),
            Self::Al => write!(f, "AL"),
            Self::Dz => write!(f, "DZ"),
            Self::As => write!(f, "AS"),
            Self::Ad => write!(f, "AD"),
            Self::Ao => write!(f, "AO"),
            Self::Ai => write!(f, "AI"),
            Self::Aq => write!(f, "AQ"),
            Self::Ag => write!(f, "AG"),
            Self::Ar => write!(f, "AR"),
            Self::Am => write!(f, "AM"),
            Self::Aw => write!(f, "AW"),
            Self::Au => write!(f, "AU"),
            Self::At => write!(f, "AT"),
            Self::Az => write!(f, "AZ"),
            Self::Bs => write!(f, "BS"),
            Self::Bh => write!(f, "BH"),
            Self::Bd => write!(f, "BD"),
            Self::Bb => write!(f, "BB"),
            Self::By => write!(f, "BY"),
            Self::Be => write!(f, "BE"),
            Self::Bz => write!(f, "BZ"),
            Self::Bj => write!(f, "BJ"),
            Self::Bm => write!(f, "BM"),
            Self::Bt => write!(f, "BT"),
            Self::Bo => write!(f, "BO"),
            Self::Bq => write!(f, "BQ"),
            Self::Ba => write!(f, "BA"),
            Self::Bw => write!(f, "BW"),
            Self::Bv => write!(f, "BV"),
            Self::Br => write!(f, "BR"),
            Self::Io => write!(f, "IO"),
            Self::Bn => write!(f, "BN"),
            Self::Bg => write!(f, "BG"),
            Self::Bf => write!(f, "BF"),
            Self::Bi => write!(f, "BI"),
            Self::Kh => write!(f, "KH"),
            Self::Cm => write!(f, "CM"),
            Self::Ca => write!(f, "CA"),
            Self::Cv => write!(f, "CV"),
            Self::Ky => write!(f, "KY"),
            Self::Cf => write!(f, "CF"),
            Self::Td => write!(f, "TD"),
            Self::Cl => write!(f, "CL"),
            Self::Cn => write!(f, "CN"),
            Self::Cx => write!(f, "CX"),
            Self::Cc => write!(f, "CC"),
            Self::Co => write!(f, "CO"),
            Self::Km => write!(f, "KM"),
            Self::Cg => write!(f, "CG"),
            Self::Cd => write!(f, "CD"),
            Self::Ck => write!(f, "CK"),
            Self::Cr => write!(f, "CR"),
            Self::Ci => write!(f, "CI"),
            Self::Hr => write!(f, "HR"),
            Self::Cu => write!(f, "CU"),
            Self::Cw => write!(f, "CW"),
            Self::Cy => write!(f, "CY"),
            Self::Cz => write!(f, "CZ"),
            Self::Dk => write!(f, "DK"),
            Self::Dj => write!(f, "DJ"),
            Self::Dm => write!(f, "DM"),
            Self::Do => write!(f, "DO"),
            Self::Ec => write!(f, "EC"),
            Self::Eg => write!(f, "EG"),
            Self::Sv => write!(f, "SV"),
            Self::Gq => write!(f, "GQ"),
            Self::Er => write!(f, "ER"),
            Self::Ee => write!(f, "EE"),
            Self::Et => write!(f, "ET"),
            Self::Fk => write!(f, "FK"),
            Self::Fo => write!(f, "FO"),
            Self::Fj => write!(f, "FJ"),
            Self::Fi => write!(f, "FI"),
            Self::Fr => write!(f, "FR"),
            Self::Gf => write!(f, "GF"),
            Self::Pf => write!(f, "PF"),
            Self::Tf => write!(f, "TF"),
            Self::Ga => write!(f, "GA"),
            Self::Gm => write!(f, "GM"),
            Self::Ge => write!(f, "GE"),
            Self::De => write!(f, "DE"),
            Self::Gh => write!(f, "GH"),
            Self::Gi => write!(f, "GI"),
            Self::Gr => write!(f, "GR"),
            Self::Gl => write!(f, "GL"),
            Self::Gd => write!(f, "GD"),
            Self::Gp => write!(f, "GP"),
            Self::Gu => write!(f, "GU"),
            Self::Gt => write!(f, "GT"),
            Self::Gg => write!(f, "GG"),
            Self::Gn => write!(f, "GN"),
            Self::Gw => write!(f, "GW"),
            Self::Gy => write!(f, "GY"),
            Self::Ht => write!(f, "HT"),
            Self::Hm => write!(f, "HM"),
            Self::Va => write!(f, "VA"),
            Self::Hn => write!(f, "HN"),
            Self::Hk => write!(f, "HK"),
            Self::Hu => write!(f, "HU"),
            Self::Is => write!(f, "IS"),
            Self::In => write!(f, "IN"),
            Self::Id => write!(f, "ID"),
            Self::Ir => write!(f, "IR"),
            Self::Iq => write!(f, "IQ"),
            Self::Ie => write!(f, "IE"),
            Self::Im => write!(f, "IM"),
            Self::Il => write!(f, "IL"),
            Self::It => write!(f, "IT"),
            Self::Jm => write!(f, "JM"),
            Self::Jp => write!(f, "JP"),
            Self::Je => write!(f, "JE"),
            Self::Jo => write!(f, "JO"),
            Self::Kz => write!(f, "KZ"),
            Self::Ke => write!(f, "KE"),
            Self::Ki => write!(f, "KI"),
            Self::Kp => write!(f, "KP"),
            Self::Kr => write!(f, "KR"),
            Self::Kw => write!(f, "KW"),
            Self::Kg => write!(f, "KG"),
            Self::La => write!(f, "LA"),
            Self::Lv => write!(f, "LV"),
            Self::Lb => write!(f, "LB"),
            Self::Ls => write!(f, "LS"),
            Self::Lr => write!(f, "LR"),
            Self::Ly => write!(f, "LY"),
            Self::Li => write!(f, "LI"),
            Self::Lt => write!(f, "LT"),
            Self::Lu => write!(f, "LU"),
            Self::Mo => write!(f, "MO"),
            Self::Mk => write!(f, "MK"),
            Self::Mg => write!(f, "MG"),
            Self::Mw => write!(f, "MW"),
            Self::My => write!(f, "MY"),
            Self::Mv => write!(f, "MV"),
            Self::Ml => write!(f, "ML"),
            Self::Mt => write!(f, "MT"),
            Self::Mh => write!(f, "MH"),
            Self::Mq => write!(f, "MQ"),
            Self::Mr => write!(f, "MR"),
            Self::Mu => write!(f, "MU"),
            Self::Yt => write!(f, "YT"),
            Self::Mx => write!(f, "MX"),
            Self::Fm => write!(f, "FM"),
            Self::Md => write!(f, "MD"),
            Self::Mc => write!(f, "MC"),
            Self::Mn => write!(f, "MN"),
            Self::Me => write!(f, "ME"),
            Self::Ms => write!(f, "MS"),
            Self::Ma => write!(f, "MA"),
            Self::Mz => write!(f, "MZ"),
            Self::Mm => write!(f, "MM"),
            Self::Na => write!(f, "NA"),
            Self::Nr => write!(f, "NR"),
            Self::Np => write!(f, "NP"),
            Self::Nl => write!(f, "NL"),
            Self::Nc => write!(f, "NC"),
            Self::Nz => write!(f, "NZ"),
            Self::Ni => write!(f, "NI"),
            Self::Ne => write!(f, "NE"),
            Self::Ng => write!(f, "NG"),
            Self::Nu => write!(f, "NU"),
            Self::Nf => write!(f, "NF"),
            Self::Mp => write!(f, "MP"),
            Self::No => write!(f, "NO"),
            Self::Om => write!(f, "OM"),
            Self::Pk => write!(f, "PK"),
            Self::Pw => write!(f, "PW"),
            Self::Ps => write!(f, "PS"),
            Self::Pa => write!(f, "PA"),
            Self::Pg => write!(f, "PG"),
            Self::Py => write!(f, "PY"),
            Self::Pe => write!(f, "PE"),
            Self::Ph => write!(f, "PH"),
            Self::Pn => write!(f, "PN"),
            Self::Pl => write!(f, "PL"),
            Self::Pt => write!(f, "PT"),
            Self::Pr => write!(f, "PR"),
            Self::Qa => write!(f, "QA"),
            Self::Re => write!(f, "RE"),
            Self::Ro => write!(f, "RO"),
            Self::Ru => write!(f, "RU"),
            Self::Rw => write!(f, "RW"),
            Self::Bl => write!(f, "BL"),
            Self::Sh => write!(f, "SH"),
            Self::Kn => write!(f, "KN"),
            Self::Lc => write!(f, "LC"),
            Self::Mf => write!(f, "MF"),
            Self::Pm => write!(f, "PM"),
            Self::Vc => write!(f, "VC"),
            Self::Ws => write!(f, "WS"),
            Self::Sm => write!(f, "SM"),
            Self::St => write!(f, "ST"),
            Self::Sa => write!(f, "SA"),
            Self::Sn => write!(f, "SN"),
            Self::Rs => write!(f, "RS"),
            Self::Sc => write!(f, "SC"),
            Self::Sl => write!(f, "SL"),
            Self::Sg => write!(f, "SG"),
            Self::Sx => write!(f, "SX"),
            Self::Sk => write!(f, "SK"),
            Self::Si => write!(f, "SI"),
            Self::Sb => write!(f, "SB"),
            Self::So => write!(f, "SO"),
            Self::Za => write!(f, "ZA"),
            Self::Gs => write!(f, "GS"),
            Self::Ss => write!(f, "SS"),
            Self::Es => write!(f, "ES"),
            Self::Lk => write!(f, "LK"),
            Self::Sd => write!(f, "SD"),
            Self::Sr => write!(f, "SR"),
            Self::Sj => write!(f, "SJ"),
            Self::Sz => write!(f, "SZ"),
            Self::Se => write!(f, "SE"),
            Self::Ch => write!(f, "CH"),
            Self::Sy => write!(f, "SY"),
            Self::Tw => write!(f, "TW"),
            Self::Tj => write!(f, "TJ"),
            Self::Tz => write!(f, "TZ"),
            Self::Th => write!(f, "TH"),
            Self::Tl => write!(f, "TL"),
            Self::Tg => write!(f, "TG"),
            Self::Tk => write!(f, "TK"),
            Self::To => write!(f, "TO"),
            Self::Tt => write!(f, "TT"),
            Self::Tn => write!(f, "TN"),
            Self::Tr => write!(f, "TR"),
            Self::Tm => write!(f, "TM"),
            Self::Tc => write!(f, "TC"),
            Self::Tv => write!(f, "TV"),
            Self::Ug => write!(f, "UG"),
            Self::Ua => write!(f, "UA"),
            Self::Ae => write!(f, "AE"),
            Self::Gb => write!(f, "GB"),
            Self::Us => write!(f, "US"),
            Self::Um => write!(f, "UM"),
            Self::Uy => write!(f, "UY"),
            Self::Uz => write!(f, "UZ"),
            Self::Vu => write!(f, "VU"),
            Self::Ve => write!(f, "VE"),
            Self::Vn => write!(f, "VN"),
            Self::Vg => write!(f, "VG"),
            Self::Vi => write!(f, "VI"),
            Self::Wf => write!(f, "WF"),
            Self::Eh => write!(f, "EH"),
            Self::Ye => write!(f, "YE"),
            Self::Zm => write!(f, "ZM"),
            Self::Zw => write!(f, "ZW"),

        }
    }
}

