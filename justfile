set shell := ["powershell", "-NoProfile", "-Command"]

server-add-web:
    spacetime server add --url https://game-server.izaforge.com iza-web

server-ping-web:
    spacetime server ping iza-web

publish-web:
    spacetime publish -p server -s iza-web bevychat --delete-data -y

gen-binds:
    spacetime generate --lang rust --out-dir bevychat-client/src/module_bindings --project-path server

bind-disco:
    spacetime generate --lang rust --out-dir disco-server/src/module_bindings --project-path server