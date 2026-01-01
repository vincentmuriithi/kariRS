#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPCM` reader - Multi-processor Communication Mode"]
pub type MPCM_R = crate::BitReader<bool>;
#[doc = "Field `MPCM` writer - Multi-processor Communication Mode"]
pub type MPCM_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `RXMODE` reader - Receiver Mode"]
pub type RXMODE_R = crate::FieldReader<u8, RXMODE_A>;
#[doc = "Receiver Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXMODE_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: CLK2x mode"]
    CLK2X = 1,
    #[doc = "2: Generic autobaud mode"]
    GENAUTO = 2,
    #[doc = "3: LIN constrained autobaud mode"]
    LINAUTO = 3,
}
impl From<RXMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXMODE_A) -> Self {
        variant as _
    }
}
impl RXMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXMODE_A {
        match self.bits {
            0 => RXMODE_A::NORMAL,
            1 => RXMODE_A::CLK2X,
            2 => RXMODE_A::GENAUTO,
            3 => RXMODE_A::LINAUTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RXMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `CLK2X`"]
    #[inline(always)]
    pub fn is_clk2x(&self) -> bool {
        *self == RXMODE_A::CLK2X
    }
    #[doc = "Checks if the value of the field is `GENAUTO`"]
    #[inline(always)]
    pub fn is_genauto(&self) -> bool {
        *self == RXMODE_A::GENAUTO
    }
    #[doc = "Checks if the value of the field is `LINAUTO`"]
    #[inline(always)]
    pub fn is_linauto(&self) -> bool {
        *self == RXMODE_A::LINAUTO
    }
}
#[doc = "Field `RXMODE` writer - Receiver Mode"]
pub type RXMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLB_SPEC, u8, RXMODE_A, 2, O>;
impl<'a, const O: u8> RXMODE_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RXMODE_A::NORMAL)
    }
    #[doc = "CLK2x mode"]
    #[inline(always)]
    pub fn clk2x(self) -> &'a mut W {
        self.variant(RXMODE_A::CLK2X)
    }
    #[doc = "Generic autobaud mode"]
    #[inline(always)]
    pub fn genauto(self) -> &'a mut W {
        self.variant(RXMODE_A::GENAUTO)
    }
    #[doc = "LIN constrained autobaud mode"]
    #[inline(always)]
    pub fn linauto(self) -> &'a mut W {
        self.variant(RXMODE_A::LINAUTO)
    }
}
#[doc = "Field `ODME` reader - Open Drain Mode Enable"]
pub type ODME_R = crate::BitReader<bool>;
#[doc = "Field `ODME` writer - Open Drain Mode Enable"]
pub type ODME_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `SFDEN` reader - Start Frame Detection Enable"]
pub type SFDEN_R = crate::BitReader<bool>;
#[doc = "Field `SFDEN` writer - Start Frame Detection Enable"]
pub type SFDEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `RXEN` reader - Reciever enable"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Reciever enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    pub fn mpcm(&self) -> MPCM_R {
        MPCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Receiver Mode"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn odme(&self) -> ODME_R {
        ODME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start Frame Detection Enable"]
    #[inline(always)]
    pub fn sfden(&self) -> SFDEN_R {
        SFDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reciever enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-processor Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpcm(&mut self) -> MPCM_W<0> {
        MPCM_W::new(self)
    }
    #[doc = "Bits 1:2 - Receiver Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxmode(&mut self) -> RXMODE_W<1> {
        RXMODE_W::new(self)
    }
    #[doc = "Bit 3 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn odme(&mut self) -> ODME_W<3> {
        ODME_W::new(self)
    }
    #[doc = "Bit 4 - Start Frame Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfden(&mut self) -> SFDEN_W<4> {
        SFDEN_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<6> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 7 - Reciever enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<7> {
        RXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
