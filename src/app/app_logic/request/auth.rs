use crate::app::app::App;
use crate::utils::next_auth;
use crate::request::auth::Auth::{BasicAuth, BearerToken, NoAuth};

impl App<'_> {
    pub fn modify_request_auth(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            selected_request.auth = next_auth(&selected_request.auth);
        }

        self.save_collection_to_file(selected_request_index.0);
        self.load_request_auth_param_tab();
    }

    pub fn select_request_auth_input_text(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();
        let selected_request = local_selected_request.read();

        match selected_request.auth {
            NoAuth => {}
            BasicAuth(_, _) => match self.auth_text_input_selection.selected {
                0 => self.edit_request_auth_username_state(),
                1 => self.edit_request_auth_password_state(),
                _ => {}
            },
            BearerToken(_) => {
                if self.auth_text_input_selection.selected == 0 {
                    self.edit_request_auth_bearer_token_state()
                }
            }
        }
    }

    pub fn modify_request_auth_basic_username(&mut self) {
        let input_text = self.auth_basic_username_text_input.text.clone();

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            if let BasicAuth(_, password) = &selected_request.auth {
                selected_request.auth = BasicAuth(input_text, password.to_string());
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn modify_request_auth_basic_password(&mut self) {
        let input_text = self.auth_basic_password_text_input.text.clone();

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            if let BasicAuth(username, _) = &selected_request.auth {
                selected_request.auth = BasicAuth(username.to_string(), input_text);
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn modify_request_auth_bearer_token(&mut self) {
        let input_text = self.auth_bearer_token_text_input.text.clone();

        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            if let BearerToken(_) = &selected_request.auth {
                selected_request.auth = BearerToken(input_text);
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }
}
