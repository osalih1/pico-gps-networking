; ModuleID = 'autocfg_fad4e72f7958b41d_2.574a6fbfa9770081-cgu.0'
source_filename = "autocfg_fad4e72f7958b41d_2.574a6fbfa9770081-cgu.0"
target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv6m-none-unknown-eabi"

@alloc_f93507f8ba4b5780b14b2c2584609be0 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\F0?" }>, align 8
@alloc_ef0a1f828f3393ef691f2705e817091c = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\00@" }>, align 8

; core::f64::<impl f64>::total_cmp
; Function Attrs: inlinehint nounwind
define internal i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h022b1f70796aa40eE"(ptr align 8 %self, ptr align 8 %other) unnamed_addr #0 {
start:
  %right = alloca [8 x i8], align 8
  %left = alloca [8 x i8], align 8
  %self1 = load double, ptr %self, align 8
  %_4 = bitcast double %self1 to i64
  store i64 %_4, ptr %left, align 8
  %self2 = load double, ptr %other, align 8
  %_7 = bitcast double %self2 to i64
  store i64 %_7, ptr %right, align 8
  %_13 = load i64, ptr %left, align 8
  %_12 = ashr i64 %_13, 63
  %_10 = lshr i64 %_12, 1
  %0 = load i64, ptr %left, align 8
  %1 = xor i64 %0, %_10
  store i64 %1, ptr %left, align 8
  %_18 = load i64, ptr %right, align 8
  %_17 = ashr i64 %_18, 63
  %_15 = lshr i64 %_17, 1
  %2 = load i64, ptr %right, align 8
  %3 = xor i64 %2, %_15
  store i64 %3, ptr %right, align 8
  %_21 = load i64, ptr %left, align 8
  %_22 = load i64, ptr %right, align 8
  %4 = icmp sgt i64 %_21, %_22
  %5 = zext i1 %4 to i8
  %6 = icmp slt i64 %_21, %_22
  %7 = zext i1 %6 to i8
  %_0 = sub nsw i8 %5, %7
  ret i8 %_0
}

; autocfg_fad4e72f7958b41d_2::probe
; Function Attrs: nounwind
define dso_local void @_ZN26autocfg_fad4e72f7958b41d_25probe17hec6eec645b6143fcE() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::total_cmp
  %_1 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h022b1f70796aa40eE"(ptr align 8 @alloc_f93507f8ba4b5780b14b2c2584609be0, ptr align 8 @alloc_ef0a1f828f3393ef691f2705e817091c) #2
  ret void
}

attributes #0 = { inlinehint nounwind "frame-pointer"="all" "target-cpu"="generic" "target-features"="+strict-align,+atomics-32" }
attributes #1 = { nounwind "frame-pointer"="all" "target-cpu"="generic" "target-features"="+strict-align,+atomics-32" }
attributes #2 = { nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.81.0 (eeb90cda1 2024-09-04)"}
