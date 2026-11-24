
declare const foo: { bar: () => { baz: number } | null | undefined };
foo.bar?.()?.baz;
      
