#[doc = "Register `MSTATUS` reader"]
pub struct R(crate::R<MSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTATUS` writer"]
pub struct W(crate::W<MSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTATUS_SPEC>;
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
impl From<crate::W<MSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSSTATE` reader - Bus State"]
pub type BUSSTATE_R = crate::FieldReader<u8, BUSSTATE_A>;
#[doc = "Bus State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUSSTATE_A {
    #[doc = "0: Unknown Bus State"]
    UNKNOWN = 0,
    #[doc = "1: Bus is Idle"]
    IDLE = 1,
    #[doc = "2: This Module Controls The Bus"]
    OWNER = 2,
    #[doc = "3: The Bus is Busy"]
    BUSY = 3,
}
impl From<BUSSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: BUSSTATE_A) -> Self {
        variant as _
    }
}
impl BUSSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSSTATE_A {
        match self.bits {
            0 => BUSSTATE_A::UNKNOWN,
            1 => BUSSTATE_A::IDLE,
            2 => BUSSTATE_A::OWNER,
            3 => BUSSTATE_A::BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNKNOWN`"]
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        *self == BUSSTATE_A::UNKNOWN
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `OWNER`"]
    #[inline(always)]
    pub fn is_owner(&self) -> bool {
        *self == BUSSTATE_A::OWNER
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSSTATE_A::BUSY
    }
}
#[doc = "Field `BUSSTATE` writer - Bus State"]
pub type BUSSTATE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, MSTATUS_SPEC, u8, BUSSTATE_A, 2, O>;
impl<'a, const O: u8> BUSSTATE_W<'a, O> {
    #[doc = "Unknown Bus State"]
    #[inline(always)]
    pub fn unknown(self) -> &'a mut W {
        self.variant(BUSSTATE_A::UNKNOWN)
    }
    #[doc = "Bus is Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(BUSSTATE_A::IDLE)
    }
    #[doc = "This Module Controls The Bus"]
    #[inline(always)]
    pub fn owner(self) -> &'a mut W {
        self.variant(BUSSTATE_A::OWNER)
    }
    #[doc = "The Bus is Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(BUSSTATE_A::BUSY)
    }
}
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSTATUS_SPEC, bool, O>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost"]
pub type ARBLOST_R = crate::BitReader<bool>;
#[doc = "Field `ARBLOST` writer - Arbitration Lost"]
pub type ARBLOST_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSTATUS_SPEC, bool, O>;
#[doc = "Field `RXACK` reader - Received Acknowledge"]
pub type RXACK_R = crate::BitReader<bool>;
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub type CLKHOLD_R = crate::BitReader<bool>;
#[doc = "Field `CLKHOLD` writer - Clock Hold"]
pub type CLKHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSTATUS_SPEC, bool, O>;
#[doc = "Field `WIF` reader - Write Interrupt Flag"]
pub type WIF_R = crate::BitReader<bool>;
#[doc = "Field `WIF` writer - Write Interrupt Flag"]
pub type WIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSTATUS_SPEC, bool, O>;
#[doc = "Field `RIF` reader - Read Interrupt Flag"]
pub type RIF_R = crate::BitReader<bool>;
#[doc = "Field `RIF` writer - Read Interrupt Flag"]
pub type RIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Bus State"]
    #[inline(always)]
    pub fn busstate(&self) -> BUSSTATE_R {
        BUSSTATE_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received Acknowledge"]
    #[inline(always)]
    pub fn rxack(&self) -> RXACK_R {
        RXACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Interrupt Flag"]
    #[inline(always)]
    pub fn wif(&self) -> WIF_R {
        WIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read Interrupt Flag"]
    #[inline(always)]
    pub fn rif(&self) -> RIF_R {
        RIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Bus State"]
    #[inline(always)]
    #[must_use]
    pub fn busstate(&mut self) -> BUSSTATE_W<0> {
        BUSSTATE_W::new(self)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<2> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 3 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ARBLOST_W<3> {
        ARBLOST_W::new(self)
    }
    #[doc = "Bit 5 - Clock Hold"]
    #[inline(always)]
    #[must_use]
    pub fn clkhold(&mut self) -> CLKHOLD_W<5> {
        CLKHOLD_W::new(self)
    }
    #[doc = "Bit 6 - Write Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wif(&mut self) -> WIF_W<6> {
        WIF_W::new(self)
    }
    #[doc = "Bit 7 - Read Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RIF_W<7> {
        RIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstatus](index.html) module"]
pub struct MSTATUS_SPEC;
impl crate::RegisterSpec for MSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mstatus::R](R) reader structure"]
impl crate::Readable for MSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstatus::W](W) writer structure"]
impl crate::Writable for MSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTATUS to value 0"]
impl crate::Resettable for MSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
