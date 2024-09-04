#!/usr/bin/env python
# encoding: utf-8

from flask import Flask, request, jsonify
import random
import sys
from core import (
    gen_email_auth_proof,
)

app = Flask(__name__)


@app.route("/prove/email_auth", methods=["POST"])
def prove_email_auth():
    req = request.get_json()
    input = req["input"]
    print(input)
    print(type(input))
    nonce = random.randint(
        0,
        sys.maxsize,
    )
    proof = gen_email_auth_proof(str(nonce), True, input)
    return jsonify(proof)


if __name__ == "__main__":
    from waitress import serve

    serve(app, host="0.0.0.0", port=8080)
