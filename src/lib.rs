#![no_std]

pub trait Trainer {
    type Trainee: ?Sized;
    type OutputGradient;
    type InputGradient;

    fn train_propogate(
        &self,
        trainee: &mut Self::Trainee,
        output_gradient: &Self::OutputGradient,
    ) -> Self::InputGradient {
        self.train(trainee, output_gradient);
        self.propogate(output_gradient)
    }
    fn train(&self, trainee: &mut Self::Trainee, output_gradient: &Self::OutputGradient);
    fn propogate(&self, output_gradient: &Self::OutputGradient) -> Self::InputGradient;
}

pub trait Trainable {
    type Input;
    type Output;
    type Trainer: Trainer<Trainee = Self>;

    fn transform(&self, input: Self::Input) -> Self::Output;
}
