
declare const foo:
  (() => { bar: number } | null | undefined) | null | undefined;
!foo?.()?.bar;
      
