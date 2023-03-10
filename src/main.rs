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

    // 6. Structural / Adapter
    patterns::structural::adapter::structural_adapter();

    // 7. Structural / Bridge
    patterns::structural::bridge::structural_bridge();

    // 8. Structural / Composite
    patterns::structural::composite::structural_composite();

    // 9. Structural / Decorator
    patterns::structural::decorator::structural_decorator();

    // 10. Structural / Facade
    patterns::structural::facade::structural_facade();

    // 11. Structural / Flyweight
    patterns::structural::flyweight::structural_flyweight();

    // 12. Structural / Proxy
    patterns::structural::proxy::structural_proxy();

    // 13. Behavioral / Chain of responsibility
    patterns::behavioral::chain_of_responsibility::behavioral_chain_of_responsibility();

    // 14. Behavioral / Command
    patterns::behavioral::command::behavioral_command();

    // 15. Behavioral / Interpreter
    patterns::behavioral::interpreter::behavioral_interpreter();

    // 16. Behavioral / Iterator
    patterns::behavioral::iterator::behavioral_iterator();

    // 17. Behavioral / Mediator
    patterns::behavioral::mediator::behavioral_mediator();

    // 18. Behavioral / Memento
    patterns::behavioral::memento::behavioral_memento();

    // 19. Behavioral / Observer
    patterns::behavioral::observer::behavioral_observer();

    // 20. Behavioral / State
    patterns::behavioral::state::behavioral_state();

    // 21. Behavioral / Strategy
    patterns::behavioral::strategy::behavioral_strategy();

    // 22. Behavioral / Template method
    patterns::behavioral::template_method::behavioral_template_method();

    // 23. Behavioral / Visitor
    patterns::behavioral::visitor::behavioral_visitor();
}
