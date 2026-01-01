#[doc = "Register `LOW` reader"]
pub struct R(crate::R<LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLLCK_SUT_CKSEL` reader - Select Clock Source"]
pub type PLLCK_SUT_CKSEL_R = crate::FieldReader<u8, PLLCK_SUT_CKSEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLCK_SUT_CKSEL_A {
    #[doc = "1: PLL Clock; Start-up time: 1K CK + 0 ms"]
    PLLCLK_1KCK_0MS = 1,
    #[doc = "17: PLL Clock; Start-up time: 1K CK + 4 ms"]
    PLLCLK_1KCK_4MS = 17,
    #[doc = "33: PLL Clock; Start-up time: 1K CK + 64 ms"]
    PLLCLK_1KCK_64MS = 33,
    #[doc = "49: PLL Clock; Start-up time: 16K CK + 64 ms"]
    PLLCLK_16KCK_64MS = 49,
    #[doc = "128: Ext. Clock; Start-up time: 6 CK + 0 ms"]
    EXTCLK_6CK_0MS = 128,
    #[doc = "129: Int. RC Osc. 1 MHz; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_1MHZ_6CK_0MS = 129,
    #[doc = "130: Int. RC Osc. 2 MHz; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_2MHZ_6CK_0MS = 130,
    #[doc = "131: Int. RC Osc. 4 MHz; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_4MHZ_6CK_0MS = 131,
    #[doc = "132: Int. RC Osc. 8 MHz; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_8MHZ_6CK_0MS = 132,
    #[doc = "133: Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 0 ms"]
    EXTRCOSC_XX_0MHZ9_18CK_0MS = 133,
    #[doc = "134: Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 0 ms"]
    EXTRCOSC_0MHZ9_3MHZ_18CK_0MS = 134,
    #[doc = "135: Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 0 ms"]
    EXTRCOSC_3MHZ_8MHZ_18CK_0MS = 135,
    #[doc = "136: Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 0 ms"]
    EXTRCOSC_8MHZ_12MHZ_18CK_0MS = 136,
    #[doc = "137: Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4 ms"]
    EXTLOFXTAL_1KCK_4MS = 137,
    #[doc = "138: Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 4 ms"]
    EXTLOFXTALRES_258CK_4MS = 138,
    #[doc = "139: Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 64 ms"]
    EXTLOFXTALRES_1KCK_64MS = 139,
    #[doc = "140: Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 4 ms"]
    EXTMEDFXTALRES_258CK_4MS = 140,
    #[doc = "141: Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 64 ms"]
    EXTMEDFXTALRES_1KCK_64MS = 141,
    #[doc = "142: Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 4 ms"]
    EXTHIFXTALRES_258CK_4MS = 142,
    #[doc = "143: Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 64 ms"]
    EXTHIFXTALRES_1KCK_64MS = 143,
    #[doc = "144: Ext. Clock; Start-up time: 6 CK + 4 ms"]
    EXTCLK_6CK_4MS = 144,
    #[doc = "145: Int. RC Osc. 1 MHz; Start-up time: 6 CK + 4 ms"]
    INTRCOSC_1MHZ_6CK_4MS = 145,
    #[doc = "146: Int. RC Osc. 2 MHz; Start-up time: 6 CK + 4 ms"]
    INTRCOSC_2MHZ_6CK_4MS = 146,
    #[doc = "147: Int. RC Osc. 4 MHz; Start-up time: 6 CK + 4 ms"]
    INTRCOSC_4MHZ_6CK_4MS = 147,
    #[doc = "148: Int. RC Osc. 8 MHz; Start-up time: 6 CK + 4 ms"]
    INTRCOSC_8MHZ_6CK_4MS = 148,
    #[doc = "149: Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 4 ms"]
    EXTRCOSC_XX_0MHZ9_18CK_4MS = 149,
    #[doc = "150: Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 4 ms"]
    EXTRCOSC_0MHZ9_3MHZ_18CK_4MS = 150,
    #[doc = "151: Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 4 ms"]
    EXTRCOSC_3MHZ_8MHZ_18CK_4MS = 151,
    #[doc = "152: Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 4 ms"]
    EXTRCOSC_8MHZ_12MHZ_18CK_4MS = 152,
    #[doc = "153: Ext. Low-Freq. Crystal; Start-up time: 1K CK + 64 ms"]
    EXTLOFXTAL_1KCK_64MS = 153,
    #[doc = "154: Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 64 ms"]
    EXTLOFXTALRES_258CK_64MS = 154,
    #[doc = "155: Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 0 ms"]
    EXTLOFXTALRES_16KCK_0MS = 155,
    #[doc = "156: Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 64 ms"]
    EXTMEDFXTALRES_258CK_64MS = 156,
    #[doc = "157: Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 0 ms"]
    EXTMEDFXTALRES_16KCK_0MS = 157,
    #[doc = "158: Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 64 ms"]
    EXTHIFXTALRES_258CK_64MS = 158,
    #[doc = "159: Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 0 ms"]
    EXTHIFXTALRES_16KCK_0MS = 159,
    #[doc = "160: Ext. Clock; Start-up time: 6 CK + 64 ms"]
    EXTCLK_6CK_64MS = 160,
    #[doc = "161: Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms"]
    INTRCOSC_1MHZ_6CK_64MS = 161,
    #[doc = "162: Int. RC Osc. 2 MHz; Start-up time: 6 CK + 64 ms"]
    INTRCOSC_2MHZ_6CK_64MS = 162,
    #[doc = "163: Int. RC Osc. 4 MHz; Start-up time: 6 CK + 64 ms"]
    INTRCOSC_4MHZ_6CK_64MS = 163,
    #[doc = "164: Int. RC Osc. 8 MHz; Start-up time: 6 CK + 64 ms"]
    INTRCOSC_8MHZ_6CK_64MS = 164,
    #[doc = "165: Ext. RC Osc. - 0.9 MHz; Start-up time: 18 CK + 64 ms"]
    EXTRCOSC_XX_0MHZ9_18CK_64MS = 165,
    #[doc = "166: Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 18 CK + 64 ms"]
    EXTRCOSC_0MHZ9_3MHZ_18CK_64MS = 166,
    #[doc = "167: Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 18 CK + 64 ms"]
    EXTRCOSC_3MHZ_8MHZ_18CK_64MS = 167,
    #[doc = "168: Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 64 ms"]
    EXTRCOSC_8MHZ_12MHZ_18CK_64MS = 168,
    #[doc = "169: Ext. Low-Freq. Crystal; Start-up time: 32K CK + 64 ms"]
    EXTLOFXTAL_32KCK_64MS = 169,
    #[doc = "170: Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 0 ms"]
    EXTLOFXTALRES_1KCK_0MS = 170,
    #[doc = "171: Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 4 ms"]
    EXTLOFXTALRES_16KCK_4MS = 171,
    #[doc = "172: Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 0 ms"]
    EXTMEDFXTALRES_1KCK_0MS = 172,
    #[doc = "173: Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 4 ms"]
    EXTMEDFXTALRES_16KCK_4MS = 173,
    #[doc = "174: Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 0 ms"]
    EXTHIFXTALRES_1KCK_0MS = 174,
    #[doc = "175: Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 4 ms"]
    EXTHIFXTALRES_16KCK_4MS = 175,
    #[doc = "181: Ext. RC Osc. - 0.9 MHz; Start-up time: 6 CK + 4 ms"]
    EXTRCOSC_XX_0MHZ9_6CK_4MS = 181,
    #[doc = "182: Ext. RC Osc. 0.9 MHz - 3.0 MHz; Start-up time: 6 CK + 4 ms"]
    EXTRCOSC_0MHZ9_3MHZ_6CK_4MS = 182,
    #[doc = "183: Ext. RC Osc. 3.0 MHz - 8.0 MHz; Start-up time: 6 CK + 4 ms"]
    EXTRCOSC_3MHZ_8MHZ_6CK_4MS = 183,
    #[doc = "184: Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 6 CK + 4 ms"]
    EXTRCOSC_8MHZ_12MHZ_6CK_4MS = 184,
    #[doc = "186: Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 4 ms"]
    EXTLOFXTALRES_1KCK_4MS = 186,
    #[doc = "187: Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 64 ms"]
    EXTLOFXTALRES_16KCK_64MS = 187,
    #[doc = "188: Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 4 ms"]
    EXTMEDFXTALRES_1KCK_4MS = 188,
    #[doc = "189: Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 64 ms"]
    EXTMEDFXTALRES_16KCK_64MS = 189,
    #[doc = "190: Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 4 ms"]
    EXTHIFXTALRES_1KCK_4MS = 190,
    #[doc = "191: Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 64 ms"]
    EXTHIFXTALRES_16KCK_64MS = 191,
}
impl From<PLLCK_SUT_CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLCK_SUT_CKSEL_A) -> Self {
        variant as _
    }
}
impl PLLCK_SUT_CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLCK_SUT_CKSEL_A> {
        match self.bits {
            1 => Some(PLLCK_SUT_CKSEL_A::PLLCLK_1KCK_0MS),
            17 => Some(PLLCK_SUT_CKSEL_A::PLLCLK_1KCK_4MS),
            33 => Some(PLLCK_SUT_CKSEL_A::PLLCLK_1KCK_64MS),
            49 => Some(PLLCK_SUT_CKSEL_A::PLLCLK_16KCK_64MS),
            128 => Some(PLLCK_SUT_CKSEL_A::EXTCLK_6CK_0MS),
            129 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_0MS),
            130 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_0MS),
            131 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_0MS),
            132 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_0MS),
            133 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_0MS),
            134 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_0MS),
            135 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_0MS),
            136 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_0MS),
            137 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTAL_1KCK_4MS),
            138 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_258CK_4MS),
            139 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_1KCK_64MS),
            140 => Some(PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_258CK_4MS),
            141 => Some(PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_64MS),
            142 => Some(PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_258CK_4MS),
            143 => Some(PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_1KCK_64MS),
            144 => Some(PLLCK_SUT_CKSEL_A::EXTCLK_6CK_4MS),
            145 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_4MS),
            146 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_4MS),
            147 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_4MS),
            148 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_4MS),
            149 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_4MS),
            150 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_4MS),
            151 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_4MS),
            152 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_4MS),
            153 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTAL_1KCK_64MS),
            154 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_258CK_64MS),
            155 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_16KCK_0MS),
            156 => Some(PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_258CK_64MS),
            157 => Some(PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_0MS),
            158 => Some(PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_258CK_64MS),
            159 => Some(PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_16KCK_0MS),
            160 => Some(PLLCK_SUT_CKSEL_A::EXTCLK_6CK_64MS),
            161 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_64MS),
            162 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_64MS),
            163 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_64MS),
            164 => Some(PLLCK_SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_64MS),
            165 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_64MS),
            166 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_64MS),
            167 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_64MS),
            168 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_64MS),
            169 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTAL_32KCK_64MS),
            170 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_1KCK_0MS),
            171 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_16KCK_4MS),
            172 => Some(PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_0MS),
            173 => Some(PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_4MS),
            174 => Some(PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_1KCK_0MS),
            175 => Some(PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_16KCK_4MS),
            181 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_6CK_4MS),
            182 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_6CK_4MS),
            183 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_6CK_4MS),
            184 => Some(PLLCK_SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_6CK_4MS),
            186 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_1KCK_4MS),
            187 => Some(PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_16KCK_64MS),
            188 => Some(PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_4MS),
            189 => Some(PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_64MS),
            190 => Some(PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_1KCK_4MS),
            191 => Some(PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_16KCK_64MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLLCLK_1KCK_0MS`"]
    #[inline(always)]
    pub fn is_pllclk_1kck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::PLLCLK_1KCK_0MS
    }
    #[doc = "Checks if the value of the field is `PLLCLK_1KCK_4MS`"]
    #[inline(always)]
    pub fn is_pllclk_1kck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::PLLCLK_1KCK_4MS
    }
    #[doc = "Checks if the value of the field is `PLLCLK_1KCK_64MS`"]
    #[inline(always)]
    pub fn is_pllclk_1kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::PLLCLK_1KCK_64MS
    }
    #[doc = "Checks if the value of the field is `PLLCLK_16KCK_64MS`"]
    #[inline(always)]
    pub fn is_pllclk_16kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::PLLCLK_16KCK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_0MS`"]
    #[inline(always)]
    pub fn is_extclk_6ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTCLK_6CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_1MHZ_6CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_1mhz_6ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_2MHZ_6CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_2mhz_6ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_4MHZ_6CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_6ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_8MHZ_6CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_XX_0MHZ9_18CK_0MS`"]
    #[inline(always)]
    pub fn is_extrcosc_xx_0mhz9_18ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_0MHZ9_3MHZ_18CK_0MS`"]
    #[inline(always)]
    pub fn is_extrcosc_0mhz9_3mhz_18ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_3MHZ_8MHZ_18CK_0MS`"]
    #[inline(always)]
    pub fn is_extrcosc_3mhz_8mhz_18ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_8MHZ_12MHZ_18CK_0MS`"]
    #[inline(always)]
    pub fn is_extrcosc_8mhz_12mhz_18ck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTAL_1KCK_4MS`"]
    #[inline(always)]
    pub fn is_extlofxtal_1kck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTAL_1KCK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTALRES_258CK_4MS`"]
    #[inline(always)]
    pub fn is_extlofxtalres_258ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_258CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTALRES_1KCK_64MS`"]
    #[inline(always)]
    pub fn is_extlofxtalres_1kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_1KCK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTMEDFXTALRES_258CK_4MS`"]
    #[inline(always)]
    pub fn is_extmedfxtalres_258ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_258CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTMEDFXTALRES_1KCK_64MS`"]
    #[inline(always)]
    pub fn is_extmedfxtalres_1kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTHIFXTALRES_258CK_4MS`"]
    #[inline(always)]
    pub fn is_exthifxtalres_258ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_258CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTHIFXTALRES_1KCK_64MS`"]
    #[inline(always)]
    pub fn is_exthifxtalres_1kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_1KCK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_4MS`"]
    #[inline(always)]
    pub fn is_extclk_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTCLK_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_1MHZ_6CK_4MS`"]
    #[inline(always)]
    pub fn is_intrcosc_1mhz_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_2MHZ_6CK_4MS`"]
    #[inline(always)]
    pub fn is_intrcosc_2mhz_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_4MHZ_6CK_4MS`"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_8MHZ_6CK_4MS`"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_XX_0MHZ9_18CK_4MS`"]
    #[inline(always)]
    pub fn is_extrcosc_xx_0mhz9_18ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_0MHZ9_3MHZ_18CK_4MS`"]
    #[inline(always)]
    pub fn is_extrcosc_0mhz9_3mhz_18ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_3MHZ_8MHZ_18CK_4MS`"]
    #[inline(always)]
    pub fn is_extrcosc_3mhz_8mhz_18ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_8MHZ_12MHZ_18CK_4MS`"]
    #[inline(always)]
    pub fn is_extrcosc_8mhz_12mhz_18ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTAL_1KCK_64MS`"]
    #[inline(always)]
    pub fn is_extlofxtal_1kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTAL_1KCK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTALRES_258CK_64MS`"]
    #[inline(always)]
    pub fn is_extlofxtalres_258ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_258CK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTALRES_16KCK_0MS`"]
    #[inline(always)]
    pub fn is_extlofxtalres_16kck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_16KCK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTMEDFXTALRES_258CK_64MS`"]
    #[inline(always)]
    pub fn is_extmedfxtalres_258ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_258CK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTMEDFXTALRES_16KCK_0MS`"]
    #[inline(always)]
    pub fn is_extmedfxtalres_16kck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTHIFXTALRES_258CK_64MS`"]
    #[inline(always)]
    pub fn is_exthifxtalres_258ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_258CK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTHIFXTALRES_16KCK_0MS`"]
    #[inline(always)]
    pub fn is_exthifxtalres_16kck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_16KCK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_64MS`"]
    #[inline(always)]
    pub fn is_extclk_6ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTCLK_6CK_64MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_1MHZ_6CK_64MS`"]
    #[inline(always)]
    pub fn is_intrcosc_1mhz_6ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_1MHZ_6CK_64MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_2MHZ_6CK_64MS`"]
    #[inline(always)]
    pub fn is_intrcosc_2mhz_6ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_2MHZ_6CK_64MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_4MHZ_6CK_64MS`"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz_6ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_4MHZ_6CK_64MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_8MHZ_6CK_64MS`"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_XX_0MHZ9_18CK_64MS`"]
    #[inline(always)]
    pub fn is_extrcosc_xx_0mhz9_18ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_18CK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_0MHZ9_3MHZ_18CK_64MS`"]
    #[inline(always)]
    pub fn is_extrcosc_0mhz9_3mhz_18ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_18CK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_3MHZ_8MHZ_18CK_64MS`"]
    #[inline(always)]
    pub fn is_extrcosc_3mhz_8mhz_18ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_18CK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_8MHZ_12MHZ_18CK_64MS`"]
    #[inline(always)]
    pub fn is_extrcosc_8mhz_12mhz_18ck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_18CK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTAL_32KCK_64MS`"]
    #[inline(always)]
    pub fn is_extlofxtal_32kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTAL_32KCK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTALRES_1KCK_0MS`"]
    #[inline(always)]
    pub fn is_extlofxtalres_1kck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_1KCK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTALRES_16KCK_4MS`"]
    #[inline(always)]
    pub fn is_extlofxtalres_16kck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_16KCK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTMEDFXTALRES_1KCK_0MS`"]
    #[inline(always)]
    pub fn is_extmedfxtalres_1kck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTMEDFXTALRES_16KCK_4MS`"]
    #[inline(always)]
    pub fn is_extmedfxtalres_16kck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTHIFXTALRES_1KCK_0MS`"]
    #[inline(always)]
    pub fn is_exthifxtalres_1kck_0ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_1KCK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTHIFXTALRES_16KCK_4MS`"]
    #[inline(always)]
    pub fn is_exthifxtalres_16kck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_16KCK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_XX_0MHZ9_6CK_4MS`"]
    #[inline(always)]
    pub fn is_extrcosc_xx_0mhz9_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_XX_0MHZ9_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_0MHZ9_3MHZ_6CK_4MS`"]
    #[inline(always)]
    pub fn is_extrcosc_0mhz9_3mhz_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_0MHZ9_3MHZ_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_3MHZ_8MHZ_6CK_4MS`"]
    #[inline(always)]
    pub fn is_extrcosc_3mhz_8mhz_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_3MHZ_8MHZ_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTRCOSC_8MHZ_12MHZ_6CK_4MS`"]
    #[inline(always)]
    pub fn is_extrcosc_8mhz_12mhz_6ck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTRCOSC_8MHZ_12MHZ_6CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTALRES_1KCK_4MS`"]
    #[inline(always)]
    pub fn is_extlofxtalres_1kck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_1KCK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTLOFXTALRES_16KCK_64MS`"]
    #[inline(always)]
    pub fn is_extlofxtalres_16kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTLOFXTALRES_16KCK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTMEDFXTALRES_1KCK_4MS`"]
    #[inline(always)]
    pub fn is_extmedfxtalres_1kck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_1KCK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTMEDFXTALRES_16KCK_64MS`"]
    #[inline(always)]
    pub fn is_extmedfxtalres_16kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTMEDFXTALRES_16KCK_64MS
    }
    #[doc = "Checks if the value of the field is `EXTHIFXTALRES_1KCK_4MS`"]
    #[inline(always)]
    pub fn is_exthifxtalres_1kck_4ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_1KCK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTHIFXTALRES_16KCK_64MS`"]
    #[inline(always)]
    pub fn is_exthifxtalres_16kck_64ms(&self) -> bool {
        *self == PLLCK_SUT_CKSEL_A::EXTHIFXTALRES_16KCK_64MS
    }
}
#[doc = "Field `CKOPT` reader - CKOPT fuse (operation dependent of CKSEL fuses)"]
pub type CKOPT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Select Clock Source"]
    #[inline(always)]
    pub fn pllck_sut_cksel(&self) -> PLLCK_SUT_CKSEL_R {
        PLLCK_SUT_CKSEL_R::new(self.bits)
    }
    #[doc = "Bit 6 - CKOPT fuse (operation dependent of CKSEL fuses)"]
    #[inline(always)]
    pub fn ckopt(&self) -> CKOPT_R {
        CKOPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low](index.html) module"]
pub struct LOW_SPEC;
impl crate::RegisterSpec for LOW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [low::R](R) reader structure"]
impl crate::Readable for LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOW to value 0"]
impl crate::Resettable for LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
