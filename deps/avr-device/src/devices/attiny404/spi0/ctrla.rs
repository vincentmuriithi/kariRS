#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable Module"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable Module"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `PRESC` reader - Prescaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: Peripheral clock / 4 if CLK2X == 0 else Peripheral clock / 2"]
    CLK_PER_4_2 = 0,
    #[doc = "1: Peripheral clock / 16 if CLK2X == 0 else Peripheral clock / 8"]
    CLK_PER_16_8 = 1,
    #[doc = "2: Peripheral clock / 64 if CLK2X == 0 else Peripheral clock / 32"]
    CLK_PER_64_32 = 2,
    #[doc = "3: Peripheral clock / 128 if CLK2X == 0 else Peripheral clock / 64"]
    CLK_PER_128_64 = 3,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::CLK_PER_4_2,
            1 => PRESC_A::CLK_PER_16_8,
            2 => PRESC_A::CLK_PER_64_32,
            3 => PRESC_A::CLK_PER_128_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_PER_4_2`"]
    #[inline(always)]
    pub fn is_clk_per_4_2(&self) -> bool {
        *self == PRESC_A::CLK_PER_4_2
    }
    #[doc = "Checks if the value of the field is `CLK_PER_16_8`"]
    #[inline(always)]
    pub fn is_clk_per_16_8(&self) -> bool {
        *self == PRESC_A::CLK_PER_16_8
    }
    #[doc = "Checks if the value of the field is `CLK_PER_64_32`"]
    #[inline(always)]
    pub fn is_clk_per_64_32(&self) -> bool {
        *self == PRESC_A::CLK_PER_64_32
    }
    #[doc = "Checks if the value of the field is `CLK_PER_128_64`"]
    #[inline(always)]
    pub fn is_clk_per_128_64(&self) -> bool {
        *self == PRESC_A::CLK_PER_128_64
    }
}
#[doc = "Field `PRESC` writer - Prescaler"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLA_SPEC, u8, PRESC_A, 2, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "Peripheral clock / 4 if CLK2X == 0 else Peripheral clock / 2"]
    #[inline(always)]
    pub fn clk_per_4_2(self) -> &'a mut W {
        self.variant(PRESC_A::CLK_PER_4_2)
    }
    #[doc = "Peripheral clock / 16 if CLK2X == 0 else Peripheral clock / 8"]
    #[inline(always)]
    pub fn clk_per_16_8(self) -> &'a mut W {
        self.variant(PRESC_A::CLK_PER_16_8)
    }
    #[doc = "Peripheral clock / 64 if CLK2X == 0 else Peripheral clock / 32"]
    #[inline(always)]
    pub fn clk_per_64_32(self) -> &'a mut W {
        self.variant(PRESC_A::CLK_PER_64_32)
    }
    #[doc = "Peripheral clock / 128 if CLK2X == 0 else Peripheral clock / 64"]
    #[inline(always)]
    pub fn clk_per_128_64(self) -> &'a mut W {
        self.variant(PRESC_A::CLK_PER_128_64)
    }
}
#[doc = "Field `CLK2X` reader - Enable Double Speed"]
pub type CLK2X_R = crate::BitReader<bool>;
#[doc = "Field `CLK2X` writer - Enable Double Speed"]
pub type CLK2X_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `MASTER` reader - Host Operation Enable"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` writer - Host Operation Enable"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `DORD` reader - Data Order Setting"]
pub type DORD_R = crate::BitReader<bool>;
#[doc = "Field `DORD` writer - Data Order Setting"]
pub type DORD_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Module"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 4 - Enable Double Speed"]
    #[inline(always)]
    pub fn clk2x(&self) -> CLK2X_R {
        CLK2X_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host Operation Enable"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Order Setting"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Module"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:2 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<1> {
        PRESC_W::new(self)
    }
    #[doc = "Bit 4 - Enable Double Speed"]
    #[inline(always)]
    #[must_use]
    pub fn clk2x(&mut self) -> CLK2X_W<4> {
        CLK2X_W::new(self)
    }
    #[doc = "Bit 5 - Host Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<5> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 6 - Data Order Setting"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<6> {
        DORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
