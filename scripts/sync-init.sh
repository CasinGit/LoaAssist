#!/bin/bash

# 설정
PRIVATE_REPO_DIR=$(pwd)
PUBLIC_REPO_NAME="LoaAssist"
PUBLIC_REPO_DIR="../loaassist-public"
TEMP_DIR="../loaassist-temp"
PUBLIC_REMOTE="git@github.com:CasinGit/LoaAssist.git"

echo "📦 [초기 설정] 비공개(main) → 공개(main) 최초 동기화 시작..."

# 클린업
rm -rf "$TEMP_DIR"
mkdir "$TEMP_DIR"

# 비공개 레포의 main 브랜치 내용 복사
git --work-tree="$TEMP_DIR" checkout main -- .

# 공개용 README 교체
cp "$PRIVATE_REPO_DIR/README.public.md" "$TEMP_DIR/README.md"
rm "$TEMP_DIR/README.public.md"

# 공개 레포가 없다면 clone
if [ ! -d "$PUBLIC_REPO_DIR" ]; then
  git clone "$PUBLIC_REMOTE" "$PUBLIC_REPO_DIR"
fi

cd "$PUBLIC_REPO_DIR"

# 공개 레포 이력 제거 및 초기화
git rm -rf . || true
cp -r "$TEMP_DIR"/* .

git checkout --orphan temp-main
git add .
git commit -m "🎉 Initial public release of LoaAssist"
git branch -M main

git remote set-url origin "$PUBLIC_REMOTE"
git push -f origin main

echo "✅ 최초 공개 레포 '$PUBLIC_REPO_NAME'로의 동기화 완료 (히스토리 초기화됨)"
