
interface Foo {
  foo: ((bar: string) => one) & ((bar: number, baz: string) => two) & (() => three);
}
      
