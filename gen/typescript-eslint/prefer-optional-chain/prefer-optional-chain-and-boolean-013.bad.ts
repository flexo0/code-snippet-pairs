
declare const bar: { baz: string };
declare const foo:
  { [k: string]: { buzz: number } | null | undefined } | null | undefined;
foo && foo[bar.baz] && foo[bar.baz].buzz;
      
