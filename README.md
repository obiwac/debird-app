# debird-app

App for instructors at sailing school [De Bird](https://www.debird.nl).

# Roadguide

|Name|Description
|-|-|
|`pwa`|The client-side PWA itself and its files.|
|`scripts`|Utility scripts to ease development.|
|`server`|Server code. This runs separately to the PWA deployment, and is used for stuff like account management.|

# Deploying

Because GitHub Pages does not allow us a wide range of options for selecting which directory to deploy our website from, we must use [this](https://gist.github.com/cobyism/4730490) trick.
The gist (hah, get it?) of it is that we create a subtree of `pwa` and then push it to the `gh-pages` branch:

```console
git subtree push --prefix pwa origin gh-pages
```
