pub struct interval
{
    pub min : f64,
    pub max : f64,
}

impl interval
{
    pub fn contains(self, x : f64) -> bool
    {
        return x >= self.min && x <= self.max;
    }

    pub fn surrounds(self, x : f64) -> bool
    {
        return x > self.min && x < self.max;
    }

    pub fn clamp(self, x : f64) -> f64
    {
        if x < self.min
        {
            return self.min;
        }

        else if x > self.max
        {
            return self.max;
        }

        else
        {
            return x;
        }
    }

}


