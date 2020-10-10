import requests
host="https://127.0.0.1:8443/index.html"
def test_tls():
    res=requests.get(host,verify=False)
    print (res.text)
if __name__ == '__main__':
    test_tls()