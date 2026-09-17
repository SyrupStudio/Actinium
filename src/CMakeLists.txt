find_package(Qt6 REQUIRED COMPONENTS Widgets)

add_executable(Actinium main.cpp)

target_link_libraries(Actinium PRIVATE Qt6::Widgets)

set_target_properties(Actinium PROPERTIES
    CXX_STANDARD 20
    CXX_STANDARD_REQUIRED ON
    CXX_EXTENSIONS OFF
)
