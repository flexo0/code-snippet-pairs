
declare const foo:
  (() => { bar: number } | null | undefined) | null | undefined;
!foo || !foo?.() || !foo?.().bar;
      
