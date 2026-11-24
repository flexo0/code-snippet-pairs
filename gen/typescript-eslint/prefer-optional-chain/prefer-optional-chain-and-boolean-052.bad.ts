
declare const foo: { bar: () => { baz: number } | null | undefined };
foo.bar && foo.bar?.() && foo.bar?.().baz && bing;
      
