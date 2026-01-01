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
#[doc = "Register `LOW` writer"]
pub struct W(crate::W<LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSEL_SUT` reader - Select Clock Source : Start-up time"]
pub type CKSEL_SUT_R = crate::FieldReader<u8, CKSEL_SUT_A>;
#[doc = "Select Clock Source : Start-up time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKSEL_SUT_A {
    #[doc = "0: Ext. Clock; Start-up time: 6 CK + 0 ms"]
    EXTCLK_6CK_0MS = 0,
    #[doc = "2: Int. RC Osc.; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_6CK_0MS = 2,
    #[doc = "3: Int. 128kHz RC Osc.; Start-up time: 6 CK + 0 ms"]
    INTRCOSC_128KHZ_6CK_0MS = 3,
    #[doc = "6: Tranceiver Oscillator; Start-up time: 258 CK + 4.1 ms"]
    TRXOSC_258CK_4MS1 = 6,
    #[doc = "7: Tranceiver Oscillator; Start-up time: 1K CK + 65 ms"]
    TRXOSC_1KCK_65MS = 7,
    #[doc = "16: Ext. Clock; Start-up time: 6 CK + 4.1 ms"]
    EXTCLK_6CK_4MS1 = 16,
    #[doc = "18: Int. RC Osc.; Start-up time: 6 CK + 4.1 ms"]
    INTRCOSC_6CK_4MS1 = 18,
    #[doc = "19: Int. 128kHz RC Osc.; Start-up time: 6 CK + 4.1 ms"]
    INTRCOSC_128KHZ_6CK_4MS1 = 19,
    #[doc = "22: Tranceiver Oscillator; Start-up time: 258 CK + 65 ms"]
    TRXOSC_258CK_65MS = 22,
    #[doc = "23: Tranceiver Oscillator; Start-up time: 16K CK + 0 ms"]
    TRXOSC_16KCK_0MS = 23,
    #[doc = "32: Ext. Clock; Start-up time: 6 CK + 65 ms"]
    EXTCLK_6CK_65MS = 32,
    #[doc = "34: Int. RC Osc.; Start-up time: 6 CK + 65 ms"]
    INTRCOSC_6CK_65MS = 34,
    #[doc = "35: Int. 128kHz RC Osc.; Start-up time: 6 CK + 65 ms"]
    INTRCOSC_128KHZ_6CK_65MS = 35,
    #[doc = "38: Tranceiver Oscillator; Start-up time: 1K CK + 0 ms"]
    TRXOSC_1KCK_0MS = 38,
    #[doc = "39: Tranceiver Oscillator; Start-up time: 16K CK + 4.1 ms"]
    TRXOSC_16KCK_4MS1 = 39,
    #[doc = "54: Tranceiver Oscillator; Start-up time: 1K CK + 4.1 ms"]
    TRXOSC_1KCK_4MS1 = 54,
    #[doc = "55: Tranceiver Oscillator; Start-up time: 16K CK + 65 ms"]
    TRXOSC_16KCK_65MS = 55,
}
impl From<CKSEL_SUT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSEL_SUT_A) -> Self {
        variant as _
    }
}
impl CKSEL_SUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKSEL_SUT_A> {
        match self.bits {
            0 => Some(CKSEL_SUT_A::EXTCLK_6CK_0MS),
            2 => Some(CKSEL_SUT_A::INTRCOSC_6CK_0MS),
            3 => Some(CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_0MS),
            6 => Some(CKSEL_SUT_A::TRXOSC_258CK_4MS1),
            7 => Some(CKSEL_SUT_A::TRXOSC_1KCK_65MS),
            16 => Some(CKSEL_SUT_A::EXTCLK_6CK_4MS1),
            18 => Some(CKSEL_SUT_A::INTRCOSC_6CK_4MS1),
            19 => Some(CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_4MS1),
            22 => Some(CKSEL_SUT_A::TRXOSC_258CK_65MS),
            23 => Some(CKSEL_SUT_A::TRXOSC_16KCK_0MS),
            32 => Some(CKSEL_SUT_A::EXTCLK_6CK_65MS),
            34 => Some(CKSEL_SUT_A::INTRCOSC_6CK_65MS),
            35 => Some(CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_65MS),
            38 => Some(CKSEL_SUT_A::TRXOSC_1KCK_0MS),
            39 => Some(CKSEL_SUT_A::TRXOSC_16KCK_4MS1),
            54 => Some(CKSEL_SUT_A::TRXOSC_1KCK_4MS1),
            55 => Some(CKSEL_SUT_A::TRXOSC_16KCK_65MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_0MS`"]
    #[inline(always)]
    pub fn is_extclk_6ck_0ms(&self) -> bool {
        *self == CKSEL_SUT_A::EXTCLK_6CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_6CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_6ck_0ms(&self) -> bool {
        *self == CKSEL_SUT_A::INTRCOSC_6CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_6CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_6ck_0ms(&self) -> bool {
        *self == CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_0MS
    }
    #[doc = "Checks if the value of the field is `TRXOSC_258CK_4MS1`"]
    #[inline(always)]
    pub fn is_trxosc_258ck_4ms1(&self) -> bool {
        *self == CKSEL_SUT_A::TRXOSC_258CK_4MS1
    }
    #[doc = "Checks if the value of the field is `TRXOSC_1KCK_65MS`"]
    #[inline(always)]
    pub fn is_trxosc_1kck_65ms(&self) -> bool {
        *self == CKSEL_SUT_A::TRXOSC_1KCK_65MS
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_4MS1`"]
    #[inline(always)]
    pub fn is_extclk_6ck_4ms1(&self) -> bool {
        *self == CKSEL_SUT_A::EXTCLK_6CK_4MS1
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_6CK_4MS1`"]
    #[inline(always)]
    pub fn is_intrcosc_6ck_4ms1(&self) -> bool {
        *self == CKSEL_SUT_A::INTRCOSC_6CK_4MS1
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_6CK_4MS1`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_6ck_4ms1(&self) -> bool {
        *self == CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_4MS1
    }
    #[doc = "Checks if the value of the field is `TRXOSC_258CK_65MS`"]
    #[inline(always)]
    pub fn is_trxosc_258ck_65ms(&self) -> bool {
        *self == CKSEL_SUT_A::TRXOSC_258CK_65MS
    }
    #[doc = "Checks if the value of the field is `TRXOSC_16KCK_0MS`"]
    #[inline(always)]
    pub fn is_trxosc_16kck_0ms(&self) -> bool {
        *self == CKSEL_SUT_A::TRXOSC_16KCK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_65MS`"]
    #[inline(always)]
    pub fn is_extclk_6ck_65ms(&self) -> bool {
        *self == CKSEL_SUT_A::EXTCLK_6CK_65MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_6CK_65MS`"]
    #[inline(always)]
    pub fn is_intrcosc_6ck_65ms(&self) -> bool {
        *self == CKSEL_SUT_A::INTRCOSC_6CK_65MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_6CK_65MS`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_6ck_65ms(&self) -> bool {
        *self == CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_65MS
    }
    #[doc = "Checks if the value of the field is `TRXOSC_1KCK_0MS`"]
    #[inline(always)]
    pub fn is_trxosc_1kck_0ms(&self) -> bool {
        *self == CKSEL_SUT_A::TRXOSC_1KCK_0MS
    }
    #[doc = "Checks if the value of the field is `TRXOSC_16KCK_4MS1`"]
    #[inline(always)]
    pub fn is_trxosc_16kck_4ms1(&self) -> bool {
        *self == CKSEL_SUT_A::TRXOSC_16KCK_4MS1
    }
    #[doc = "Checks if the value of the field is `TRXOSC_1KCK_4MS1`"]
    #[inline(always)]
    pub fn is_trxosc_1kck_4ms1(&self) -> bool {
        *self == CKSEL_SUT_A::TRXOSC_1KCK_4MS1
    }
    #[doc = "Checks if the value of the field is `TRXOSC_16KCK_65MS`"]
    #[inline(always)]
    pub fn is_trxosc_16kck_65ms(&self) -> bool {
        *self == CKSEL_SUT_A::TRXOSC_16KCK_65MS
    }
}
#[doc = "Field `CKSEL_SUT` writer - Select Clock Source : Start-up time"]
pub type CKSEL_SUT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LOW_SPEC, u8, CKSEL_SUT_A, 6, O>;
impl<'a, const O: u8> CKSEL_SUT_W<'a, O> {
    #[doc = "Ext. Clock; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn extclk_6ck_0ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::EXTCLK_6CK_0MS)
    }
    #[doc = "Int. RC Osc.; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_6ck_0ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::INTRCOSC_6CK_0MS)
    }
    #[doc = "Int. 128kHz RC Osc.; Start-up time: 6 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_6ck_0ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_0MS)
    }
    #[doc = "Tranceiver Oscillator; Start-up time: 258 CK + 4.1 ms"]
    #[inline(always)]
    pub fn trxosc_258ck_4ms1(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::TRXOSC_258CK_4MS1)
    }
    #[doc = "Tranceiver Oscillator; Start-up time: 1K CK + 65 ms"]
    #[inline(always)]
    pub fn trxosc_1kck_65ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::TRXOSC_1KCK_65MS)
    }
    #[doc = "Ext. Clock; Start-up time: 6 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extclk_6ck_4ms1(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::EXTCLK_6CK_4MS1)
    }
    #[doc = "Int. RC Osc.; Start-up time: 6 CK + 4.1 ms"]
    #[inline(always)]
    pub fn intrcosc_6ck_4ms1(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::INTRCOSC_6CK_4MS1)
    }
    #[doc = "Int. 128kHz RC Osc.; Start-up time: 6 CK + 4.1 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_6ck_4ms1(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_4MS1)
    }
    #[doc = "Tranceiver Oscillator; Start-up time: 258 CK + 65 ms"]
    #[inline(always)]
    pub fn trxosc_258ck_65ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::TRXOSC_258CK_65MS)
    }
    #[doc = "Tranceiver Oscillator; Start-up time: 16K CK + 0 ms"]
    #[inline(always)]
    pub fn trxosc_16kck_0ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::TRXOSC_16KCK_0MS)
    }
    #[doc = "Ext. Clock; Start-up time: 6 CK + 65 ms"]
    #[inline(always)]
    pub fn extclk_6ck_65ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::EXTCLK_6CK_65MS)
    }
    #[doc = "Int. RC Osc.; Start-up time: 6 CK + 65 ms"]
    #[inline(always)]
    pub fn intrcosc_6ck_65ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::INTRCOSC_6CK_65MS)
    }
    #[doc = "Int. 128kHz RC Osc.; Start-up time: 6 CK + 65 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_6ck_65ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::INTRCOSC_128KHZ_6CK_65MS)
    }
    #[doc = "Tranceiver Oscillator; Start-up time: 1K CK + 0 ms"]
    #[inline(always)]
    pub fn trxosc_1kck_0ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::TRXOSC_1KCK_0MS)
    }
    #[doc = "Tranceiver Oscillator; Start-up time: 16K CK + 4.1 ms"]
    #[inline(always)]
    pub fn trxosc_16kck_4ms1(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::TRXOSC_16KCK_4MS1)
    }
    #[doc = "Tranceiver Oscillator; Start-up time: 1K CK + 4.1 ms"]
    #[inline(always)]
    pub fn trxosc_1kck_4ms1(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::TRXOSC_1KCK_4MS1)
    }
    #[doc = "Tranceiver Oscillator; Start-up time: 16K CK + 65 ms"]
    #[inline(always)]
    pub fn trxosc_16kck_65ms(self) -> &'a mut W {
        self.variant(CKSEL_SUT_A::TRXOSC_16KCK_65MS)
    }
}
#[doc = "Field `CKOUT` reader - Clock output on PORTE7"]
pub type CKOUT_R = crate::BitReader<bool>;
#[doc = "Field `CKOUT` writer - Clock output on PORTE7"]
pub type CKOUT_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOW_SPEC, bool, O>;
#[doc = "Field `CKDIV8` reader - Divide clock by 8 internally"]
pub type CKDIV8_R = crate::BitReader<bool>;
#[doc = "Field `CKDIV8` writer - Divide clock by 8 internally"]
pub type CKDIV8_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOW_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Select Clock Source : Start-up time"]
    #[inline(always)]
    pub fn cksel_sut(&self) -> CKSEL_SUT_R {
        CKSEL_SUT_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Clock output on PORTE7"]
    #[inline(always)]
    pub fn ckout(&self) -> CKOUT_R {
        CKOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divide clock by 8 internally"]
    #[inline(always)]
    pub fn ckdiv8(&self) -> CKDIV8_R {
        CKDIV8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select Clock Source : Start-up time"]
    #[inline(always)]
    #[must_use]
    pub fn cksel_sut(&mut self) -> CKSEL_SUT_W<0> {
        CKSEL_SUT_W::new(self)
    }
    #[doc = "Bit 6 - Clock output on PORTE7"]
    #[inline(always)]
    #[must_use]
    pub fn ckout(&mut self) -> CKOUT_W<6> {
        CKOUT_W::new(self)
    }
    #[doc = "Bit 7 - Divide clock by 8 internally"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv8(&mut self) -> CKDIV8_W<7> {
        CKDIV8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low](index.html) module"]
pub struct LOW_SPEC;
impl crate::RegisterSpec for LOW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [low::R](R) reader structure"]
impl crate::Readable for LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [low::W](W) writer structure"]
impl crate::Writable for LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOW to value 0"]
impl crate::Resettable for LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
