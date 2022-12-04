# How to make a TS app

```bash
sudo apt install curl
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.8/install.sh | bash
sudo apt install nvm
source ~/.bashrc # make nvm usable
nvm install node
npm install -g create-react-app
create-react-app [project-name] --template typescript
```

# Add mantine
In `package.json`, add something like `"@mantine/core": "^5.9.0",` to the `dependencies`.


