
interface Foo {
  [foo]: ((bar: string) => one) & ((bar: number, baz: string) => two) & (() => three);
  bar: ((arg: string) => void) & ((baz: number) => Foo);
}
      
