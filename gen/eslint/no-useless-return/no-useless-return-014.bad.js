
              function foo() {
                  if (something) {
                      try {
                          bar();
                          return;
                      } catch (err) {}
                  }
              }
            
