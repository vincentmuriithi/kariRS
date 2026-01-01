#[doc = "Register `HIGH` reader"]
pub struct R(crate::R<HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIGH` writer"]
pub struct W(crate::W<HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIGH_SPEC>;
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
impl From<crate::W<HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOTRST` reader - Boot Reset vector Enabled"]
pub type BOOTRST_R = crate::BitReader<bool>;
#[doc = "Field `BOOTRST` writer - Boot Reset vector Enabled"]
pub type BOOTRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `BOOTSZ` reader - Select Boot Size"]
pub type BOOTSZ_R = crate::FieldReader<u8, BOOTSZ_A>;
#[doc = "Select Boot Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOTSZ_A {
    #[doc = "0: Boot Flash size=2048 words start address=$1800"]
    _2048W_1800 = 0,
    #[doc = "1: Boot Flash size=1024 words start address=$1C00"]
    _1024W_1C00 = 1,
    #[doc = "2: Boot Flash size=512 words start address=$1E00"]
    _512W_1E00 = 2,
    #[doc = "3: Boot Flash size=256 words start address=$1F00"]
    _256W_1F00 = 3,
}
impl From<BOOTSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTSZ_A) -> Self {
        variant as _
    }
}
impl BOOTSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTSZ_A {
        match self.bits {
            0 => BOOTSZ_A::_2048W_1800,
            1 => BOOTSZ_A::_1024W_1C00,
            2 => BOOTSZ_A::_512W_1E00,
            3 => BOOTSZ_A::_256W_1F00,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2048W_1800`"]
    #[inline(always)]
    pub fn is_2048w_1800(&self) -> bool {
        *self == BOOTSZ_A::_2048W_1800
    }
    #[doc = "Checks if the value of the field is `_1024W_1C00`"]
    #[inline(always)]
    pub fn is_1024w_1c00(&self) -> bool {
        *self == BOOTSZ_A::_1024W_1C00
    }
    #[doc = "Checks if the value of the field is `_512W_1E00`"]
    #[inline(always)]
    pub fn is_512w_1e00(&self) -> bool {
        *self == BOOTSZ_A::_512W_1E00
    }
    #[doc = "Checks if the value of the field is `_256W_1F00`"]
    #[inline(always)]
    pub fn is_256w_1f00(&self) -> bool {
        *self == BOOTSZ_A::_256W_1F00
    }
}
#[doc = "Field `BOOTSZ` writer - Select Boot Size"]
pub type BOOTSZ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, HIGH_SPEC, u8, BOOTSZ_A, 2, O>;
impl<'a, const O: u8> BOOTSZ_W<'a, O> {
    #[doc = "Boot Flash size=2048 words start address=$1800"]
    #[inline(always)]
    pub fn _2048w_1800(self) -> &'a mut W {
        self.variant(BOOTSZ_A::_2048W_1800)
    }
    #[doc = "Boot Flash size=1024 words start address=$1C00"]
    #[inline(always)]
    pub fn _1024w_1c00(self) -> &'a mut W {
        self.variant(BOOTSZ_A::_1024W_1C00)
    }
    #[doc = "Boot Flash size=512 words start address=$1E00"]
    #[inline(always)]
    pub fn _512w_1e00(self) -> &'a mut W {
        self.variant(BOOTSZ_A::_512W_1E00)
    }
    #[doc = "Boot Flash size=256 words start address=$1F00"]
    #[inline(always)]
    pub fn _256w_1f00(self) -> &'a mut W {
        self.variant(BOOTSZ_A::_256W_1F00)
    }
}
#[doc = "Field `EESAVE` reader - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_R = crate::BitReader<bool>;
#[doc = "Field `EESAVE` writer - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `WDTON` reader - Watchdog timer always on"]
pub type WDTON_R = crate::BitReader<bool>;
#[doc = "Field `WDTON` writer - Watchdog timer always on"]
pub type WDTON_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `SPIEN` reader - Serial program downloading (SPI) enabled"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - Serial program downloading (SPI) enabled"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `RSTDISBL` reader - Reset Disabled (Enable PC6 as i/o pin)"]
pub type RSTDISBL_R = crate::BitReader<bool>;
#[doc = "Field `RSTDISBL` writer - Reset Disabled (Enable PC6 as i/o pin)"]
pub type RSTDISBL_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
#[doc = "Field `DWEN` reader - Debug Wire enable"]
pub type DWEN_R = crate::BitReader<bool>;
#[doc = "Field `DWEN` writer - Debug Wire enable"]
pub type DWEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, HIGH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Boot Reset vector Enabled"]
    #[inline(always)]
    pub fn bootrst(&self) -> BOOTRST_R {
        BOOTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Select Boot Size"]
    #[inline(always)]
    pub fn bootsz(&self) -> BOOTSZ_R {
        BOOTSZ_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    pub fn eesave(&self) -> EESAVE_R {
        EESAVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog timer always on"]
    #[inline(always)]
    pub fn wdton(&self) -> WDTON_R {
        WDTON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Disabled (Enable PC6 as i/o pin)"]
    #[inline(always)]
    pub fn rstdisbl(&self) -> RSTDISBL_R {
        RSTDISBL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Wire enable"]
    #[inline(always)]
    pub fn dwen(&self) -> DWEN_R {
        DWEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Reset vector Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bootrst(&mut self) -> BOOTRST_W<0> {
        BOOTRST_W::new(self)
    }
    #[doc = "Bits 1:2 - Select Boot Size"]
    #[inline(always)]
    #[must_use]
    pub fn bootsz(&mut self) -> BOOTSZ_W<1> {
        BOOTSZ_W::new(self)
    }
    #[doc = "Bit 3 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    #[must_use]
    pub fn eesave(&mut self) -> EESAVE_W<3> {
        EESAVE_W::new(self)
    }
    #[doc = "Bit 4 - Watchdog timer always on"]
    #[inline(always)]
    #[must_use]
    pub fn wdton(&mut self) -> WDTON_W<4> {
        WDTON_W::new(self)
    }
    #[doc = "Bit 5 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<5> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 6 - Reset Disabled (Enable PC6 as i/o pin)"]
    #[inline(always)]
    #[must_use]
    pub fn rstdisbl(&mut self) -> RSTDISBL_W<6> {
        RSTDISBL_W::new(self)
    }
    #[doc = "Bit 7 - Debug Wire enable"]
    #[inline(always)]
    #[must_use]
    pub fn dwen(&mut self) -> DWEN_W<7> {
        DWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [high](index.html) module"]
pub struct HIGH_SPEC;
impl crate::RegisterSpec for HIGH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [high::R](R) reader structure"]
impl crate::Readable for HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [high::W](W) writer structure"]
impl crate::Writable for HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIGH to value 0"]
impl crate::Resettable for HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
