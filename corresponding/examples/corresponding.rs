// In this example we show that you can copy the fields with the same
// name and type from one struct to the other, by adding the attribute
// `derive_corresponding`. The attribute should be put on a module. All
// structs within this module will get the `move_corresponding` method
// to copy all the structs within the module to each other.
// If a struct derives the Default trait, you can also use `.into()` from
// another struct in the same module.

// In this example there is a datamodel module with a user module.
// The user module has the `derive_corresponding` attribute, so all
// the structs in this module can use the `move_corresponding` method
// Because structs User and UserUpdate derive Default, you can also use
// `.into()` to construct them from another struct.

// Here is the data module with the `derive_corresponding` attribute:

mod datamodel {
    use corresponding::*;

    #[derive_corresponding]
    pub mod user {
        #[derive(Default, Debug)]
        pub struct User {
            pub id: u8,
            pub name: String,
            pub country: String,
        }

        #[derive(Clone)]
        pub struct UserKey {
            pub id: u8,
        }

        pub struct UserInsert {
            pub name: String,
            pub country: String,
        }

        #[derive(Default)]
        pub struct UserUpdate {
            pub id: u8,
            pub name: Option<String>,
            pub country: Option<String>,
        }
    }
}

// And here we are going to use them. Let's pretend we have a database and
// want to create a user and update it.

use corresponding::*;
use datamodel::user::*;

fn main() {
    // The user data from user interface
    let user_insert = UserInsert {
        name: "Mark".to_string(),
        country: "NL".to_string(),
    };

    // Insert the user into the database, returning the id
    let user_key = UserKey { id: 1 };

    // Construct the complete entity by combining received key and inserted fields
    let mut user: User = user_key.into();
    user.move_corresponding(user_insert);

    // Print the created user
    println!("{user:#?}");

    // The user moved to another country
    let user_update = UserUpdate {
        id: user.id,
        country: Some("US".to_string()),
        ..Default::default()
    };

    // Update the database...

    // Construct the updated entity
    user.move_corresponding(user_update);

    // Print the updated user
    println!("{user:#?}");
}
