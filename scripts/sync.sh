#!/usr/bin/env sh
rmmp -m ../models/models.txt -t ../templates -p sql > ../outputs/install.sql
rmmp -m ../models/models.txt -t ../templates -p models > ../src/models.rs

rmmp -m ../models/forms.txt -t ../templates -p typescript > ../outputs/forms.ts
rmmp -m ../models/responses.txt -t ../templates -p typescript > ../outputs/response.ts

rmmp -m ../models/forms.txt -t ../templates -p forms > ../src/forms.rs
rmmp -m ../models/responses.txt -t ../templates -p responses > ../src/response.rs

rmmp -m ../models/models.txt -t ../templates -p dot_graph > ../outputs/modal.dot
dot -Tpng -o ../outputs/modal-uml.png ../outputs/modal.dot
rmmp -m ../models/forms.txt -t ../templates -p dot_graph > ../outputs/forms.dot
dot -Tpng -o ../outputs/forms-uml.png ../outputs/forms.dot
rmmp -m ../models/responses.txt -t ../templates -p dot_graph > ../outputs/response.dot
dot -Tpng -o ../outputs/response-uml.png ../outputs/response.dot

echo "Done"