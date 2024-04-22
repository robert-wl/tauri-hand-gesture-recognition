set "emptyVenv=.venv.empty"
set "venv=.venv"

ren ".venv.empty" ".venv"

call .venv\Scripts\activate

pip install mediapipe==0.10.11 numpy==1.26.2 opencv-python==4.9.0.80 pandas==2.0.0 scikit-learn==1.4.2 seaborn==0.13.2 matplotlib==3.9.0rc2

pause

deactivate
