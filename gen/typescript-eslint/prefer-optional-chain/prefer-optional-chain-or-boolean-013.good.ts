
declare const bar: { baz: string };
declare const foo:
  { [k: string]: { buzz: number } | null | undefined } | null | undefined;
!foo?.[bar.baz]?.buzz;
      
