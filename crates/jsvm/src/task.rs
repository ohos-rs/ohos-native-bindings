use crate::{Env, Promise, Result, ToJsValue};

pub trait Task: Send + Sized {
    type Output: Send + Sized + 'static;
    type JsValue: ToJsValue;

    fn compute(&mut self) -> Result<Self::Output>;

    fn resolve(&mut self, env: &Env, output: Self::Output) -> Result<Self::JsValue>;

    fn reject(&mut self, _env: &Env, err: crate::JsvmError) -> Result<Self::JsValue> {
        Err(err)
    }

    fn finally(self, _env: &Env) -> Result<()> {
        Ok(())
    }
}

impl Env {
    pub fn run_task<T>(&self, mut task: T) -> Result<Promise>
    where
        T: Task,
    {
        let (deferred, promise) = self.create_promise()?;
        match task.compute() {
            Ok(output) => {
                let value = task.resolve(self, output)?;
                deferred.resolve(self, &value)?;
            }
            Err(err) => {
                let value = task.reject(self, err)?;
                deferred.reject(self, &value)?;
            }
        }
        task.finally(self)?;
        Ok(promise)
    }
}
