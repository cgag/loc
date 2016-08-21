/*-
 * Copyright (c) 1997 Bruce Evans.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * $FreeBSD: src/sys/i386/isa/ipl_funcs.c,v 1.32.2.5 2002/12/17 18:04:02 sam Exp $
 * $DragonFly: src/sys/platform/vkernel/platform/ipl_funcs.c,v 1.2 2007/01/11 23:23:56 dillon Exp $
 */

#include <sys/param.h>
#include <sys/systm.h>
#include <sys/kernel.h>
#include <sys/sysctl.h>
#include <sys/proc.h>
#include <sys/interrupt.h>
#include <machine/globaldata.h>

#include <unistd.h>

/*
 * Bits in the ipending bitmap variable must be set atomically because
 * ipending may be manipulated by interrupts or other cpu's without holding
 * any locks.
 *
 * Note: setbits uses a locked or, making simple cases MP safe.
 */
#define DO_SETBITS(name, var, bits) 					\
void									\
name(void)								\
{									\
	struct mdglobaldata *gd = mdcpu;				\
	atomic_set_int_nonlocked(var, bits);				\
	atomic_set_int(&gd->mi.gd_reqflags, RQF_INTPEND);		\
	umtx_wakeup(&gd->mi.gd_reqflags, 0);				\
}									\

DO_SETBITS(setdelayed,   &gd->gd_spending, loadandclear(&gd->gd_sdelayed))

DO_SETBITS(setsoftcamnet,&gd->gd_spending, SWI_CAMNET_PENDING)
DO_SETBITS(setsoftcambio,&gd->gd_spending, SWI_CAMBIO_PENDING)
/*DO_SETBITS(setsoftunused02, &gd->gd_spending, SWI_UNUSED02_PENDING)*/
/*DO_SETBITS(setsoftunused01,   &gd->gd_spending, SWI_UNUSED01_PENDING)*/
DO_SETBITS(setsofttty,   &gd->gd_spending, SWI_TTY_PENDING)
DO_SETBITS(setsoftvm,	 &gd->gd_spending, SWI_VM_PENDING)
DO_SETBITS(setsofttq,	 &gd->gd_spending, SWI_TQ_PENDING)
DO_SETBITS(setsoftcrypto,&gd->gd_spending, SWI_CRYPTO_PENDING)

/*DO_SETBITS(schedsoftcamnet, &gd->gd_sdelayed, SWI_CAMNET_PENDING)*/
/*DO_SETBITS(schedsoftcambio, &gd->gd_sdelayed, SWI_CAMBIO_PENDING)*/
/*DO_SETBITS(schedsoftunused01, &gd->gd_sdelayed, SWI_UNUSED01_PENDING)*/
DO_SETBITS(schedsofttty, &gd->gd_sdelayed, SWI_TTY_PENDING)
/*DO_SETBITS(schedsoftvm, &gd->gd_sdelayed, SWI_VM_PENDING)*/
/*DO_SETBITS(schedsofttq, &gd->gd_sdelayed, SWI_TQ_PENDING)*/
/* YYY schedsoft what? */
