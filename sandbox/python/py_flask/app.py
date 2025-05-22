from flask import Flask, render_template, request

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html', 
                         title='Flask UV Project',
                         heading='Welcome to My Flask App',
                         message='This is a simple Flask app with Jinja2 templates, running dynamically.',
                         features=['Jinja2 Templates', 'Dynamic Request Handling', 'Docker Deployment'])

@app.route('/greet')
@app.route('/greet/<name>')
def greet(name=None):
    if name:
        message = f'Welcome, {name}, to the greeting page of this Flask app!'
    else:
        message = 'Welcome to the greeting page of this Flask app!'
    return render_template('greet.html',
                         title='Greeting Page',
                         heading='Hello, Visitor!',
                         message=message)

@app.route('/get', methods=['GET'])
def get_form():
    name = request.args.get('name')
    message = request.args.get('message')
    return render_template('get.html',
                         title='GET Form',
                         heading='Submit a GET Form',
                         message='Enter your name and message below.',
                         name=name,
                         get_message=message)

@app.route('/post', methods=['GET', 'POST'])
def post_form():
    post_data = None
    if request.method == 'POST':
        post_data = {
            'name': request.form.get('name'),
            'message': request.form.get('message')
        }
    return render_template('post.html',
                         title='POST Form',
                         heading='Submit a POST Form',
                         message='Enter your name and message below.',
                         post_data=post_data)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000, debug=True)