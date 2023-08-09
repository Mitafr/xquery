cd $PWD

cd front
yarn build
rm -rf ../back/dist
cp -R dist ../back
head -n -7 ../back/dist/index.html > tmp.txt && mv tmp.txt ../back/dist/index.html
echo "
<body>
  {% if authenticated == \"true\" %}
  <div id="app" data-authenticated="true" data-username="{{ username }}"></div>
  {% else %}
  <div id="app" data-authenticated="false"></div>
  {% endif %}
</body>

</html>" >> ../back/dist/index.html
cd ../back
cargo run