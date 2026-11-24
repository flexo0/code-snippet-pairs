Decimal(" nan ")          # Decimal(" nan ") 
Decimal(" +nan ")         # Decimal(" +nan ")
# In this one case, " -nan ", the fix has to be
# `Decimal(" nan ")`` because `Decimal("-nan") != Decimal(float("-nan"))`
