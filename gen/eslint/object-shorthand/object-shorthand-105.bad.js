
                function foo() {
                    var x = {
                        x: () => {
                            ({ y: () => { foo; } });
                            this;
                        }
                    };
                }
            
