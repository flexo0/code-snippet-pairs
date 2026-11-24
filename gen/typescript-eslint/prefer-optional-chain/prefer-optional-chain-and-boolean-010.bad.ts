
declare const foo:
  | { bar: { baz: { buzz: number } | null | undefined } | null | undefined }
  | null
  | undefined;
foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz;
      
