mod patterns;

fn main() {
    println!("Design patterns");

    // 1. Creational / Abstract Factory
    patterns::creational::abstract_factory::crational_abstract_factory();

    // 2. Creational / Builder
    patterns::creational::builder::creational_builder();

    // 3. Creational / Factory method
    patterns::creational::factory_method::creational_factory_method();

    // 4. Creational / Prototype
    patterns::creational::prototype::creational_prototype();

    // 5. Creational / Singleton
    patterns::creational::singleton::creational_singleton();
}
